use std::sync::Arc;
use axum::{
    Router,
    routing::{get, post},
    extract::{State, Path, Json},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::state::AppState;
use crate::models::order::{OrderRequest, OrderAction, TradeSource, AccountScope};
use crate::engine::dispatch;

pub type SharedState = Arc<AppState>;

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    message: String,
}

impl<T: Serialize> ApiResponse<T> {
    fn ok(data: T) -> Self {
        Self { success: true, data: Some(data), message: "ok".to_string() }
    }
    fn err(msg: &str) -> Self {
        Self { success: false, data: None, message: msg.to_string() }
    }
}

/// Build router cho REST API
pub fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/accounts", get(get_accounts))
        .route("/api/bots", get(get_bots))
        .route("/api/positions", get(get_positions))
        .route("/api/order", post(post_order))
        .route("/api/bots/:id/enable", post(enable_bot))
        .route("/api/bots/:id/disable", post(disable_bot))
        .route("/api/autotrade/on", post(autotrade_on))
        .route("/api/autotrade/off", post(autotrade_off))
        .route("/api/report", get(get_report))
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}

/// Auth middleware - kiểm tra Bearer token theo từng ApiClient
async fn auth_middleware(
    State(state): State<SharedState>,
    headers: HeaderMap,
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> impl IntoResponse {
    // /health không cần auth
    if req.uri().path() == "/health" {
        return next.run(req).await;
    }

    let auth = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !auth.starts_with("Bearer ") {
        return (StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({"success": false, "error": "Missing Bearer token"}))).into_response();
    }

    let token = &auth["Bearer ".len()..];
    match state.authenticate_key(token).await {
        Some(client) => {
            if !client.enabled {
                return (StatusCode::FORBIDDEN,
                    axum::Json(serde_json::json!({"success": false, "error": "API key is disabled"}))).into_response();
            }
            // Ghi usage và inject source vào request extensions
            let client_id = client.id.clone();
            let source = client.source.clone();
            // Tăng counter usage (fire & forget)
            let db = state.db.clone();
            let cid = client_id.clone();
            tokio::spawn(async move {
                let _ = crate::storage::increment_client_usage(&db, &cid).await;
            });
            // Inject client info vào request headers
            req.extensions_mut().insert(source);
            next.run(req).await
        }
        None => {
            (StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"success": false, "error": "Invalid or disabled API key"}))).into_response()
        }
    }
}

// ── Handlers ─────────────────────────────────────────────────────────────────

async fn health() -> impl IntoResponse {
    axum::Json(serde_json::json!({
        "status": "ok",
        "service": "iZFx.Trade",
        "version": "2.0.0"
    }))
}

async fn get_accounts(State(state): State<SharedState>) -> impl IntoResponse {
    let accounts = state.accounts.read().await;
    let list: Vec<_> = accounts.values().cloned().collect();
    axum::Json(ApiResponse::ok(list))
}

async fn get_bots(State(state): State<SharedState>) -> impl IntoResponse {
    let bots = state.bots.read().await;
    let list: Vec<_> = bots.values().cloned().collect();
    axum::Json(ApiResponse::ok(list))
}

async fn get_positions(State(state): State<SharedState>) -> impl IntoResponse {
    let positions = state.get_open_positions().await;
    axum::Json(ApiResponse::ok(positions))
}

#[derive(Deserialize)]
struct OrderBody {
    bot_id: String,
    action: String,
    account_scope: Option<String>,
    account_ids: Option<Vec<i64>>,
    symbol: Option<String>,
    side: Option<String>,
    volume: Option<f64>,
    sl: Option<f64>,
    tp: Option<f64>,
}

async fn post_order(
    State(state): State<SharedState>,
    Json(body): Json<OrderBody>,
) -> impl IntoResponse {
    let action = match body.action.to_uppercase().as_str() {
        "OPEN" => OrderAction::Open,
        "CLOSE" => OrderAction::Close,
        "CLOSE_ALL" => OrderAction::CloseAll,
        "ENABLE_BOT" => OrderAction::EnableBot,
        "DISABLE_BOT" => OrderAction::DisableBot,
        "ENABLE_AUTOTRADE" => OrderAction::EnableAutotrade,
        "DISABLE_AUTOTRADE" => OrderAction::DisableAutotrade,
        _ => return axum::Json(ApiResponse::<()>::err("Invalid action")).into_response(),
    };

    let scope = match body.account_scope.as_deref().unwrap_or("all") {
        "all" => AccountScope::All,
        "single" => AccountScope::Single,
        "list" => AccountScope::List,
        _ => AccountScope::All,
    };

    let mut req = OrderRequest::new(TradeSource::Api, body.bot_id, action);
    req.account_scope = scope;
    req.account_ids = body.account_ids.unwrap_or_default();
    req.symbol = body.symbol.map(|s| s.to_uppercase());
    req.volume = body.volume;
    req.sl = body.sl;
    req.tp = body.tp;

    if let Some(side_str) = body.side {
        req.side = match side_str.to_lowercase().as_str() {
            "buy" => Some(crate::models::order::TradeSide::Buy),
            "sell" => Some(crate::models::order::TradeSide::Sell),
            _ => None,
        };
    }

    match dispatch(state, req).await {
        Ok(result) => axum::Json(serde_json::json!({
            "success": result.error_count == 0,
            "request_id": result.request_id,
            "executed": result.success_count,
            "errors": result.error_count,
            "messages": result.messages,
        })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn enable_bot(
    State(state): State<SharedState>,
    Path(bot_id): Path<String>,
) -> impl IntoResponse {
    let req = OrderRequest::new(TradeSource::Api, bot_id, OrderAction::EnableBot);
    match dispatch(state, req).await {
        Ok(r) => axum::Json(ApiResponse::ok(r.messages)),
        Err(e) => axum::Json(ApiResponse::<Vec<String>>::err(&e.to_string())),
    }
}

async fn disable_bot(
    State(state): State<SharedState>,
    Path(bot_id): Path<String>,
) -> impl IntoResponse {
    let req = OrderRequest::new(TradeSource::Api, bot_id, OrderAction::DisableBot);
    match dispatch(state, req).await {
        Ok(r) => axum::Json(ApiResponse::ok(r.messages)),
        Err(e) => axum::Json(ApiResponse::<Vec<String>>::err(&e.to_string())),
    }
}

async fn autotrade_on(State(state): State<SharedState>) -> impl IntoResponse {
    let req = OrderRequest::new(TradeSource::Api, "system".to_string(), OrderAction::EnableAutotrade);
    match dispatch(state, req).await {
        Ok(r) => axum::Json(ApiResponse::ok(r.messages)),
        Err(e) => axum::Json(ApiResponse::<Vec<String>>::err(&e.to_string())),
    }
}

async fn autotrade_off(State(state): State<SharedState>) -> impl IntoResponse {
    let req = OrderRequest::new(TradeSource::Api, "system".to_string(), OrderAction::DisableAutotrade);
    match dispatch(state, req).await {
        Ok(r) => axum::Json(ApiResponse::ok(r.messages)),
        Err(e) => axum::Json(ApiResponse::<Vec<String>>::err(&e.to_string())),
    }
}

async fn get_report(State(state): State<SharedState>) -> impl IntoResponse {
    let accounts = state.accounts.read().await;
    let bots = state.bots.read().await;
    let positions = state.get_open_positions().await;

    axum::Json(serde_json::json!({
        "accounts": accounts.values().collect::<Vec<_>>(),
        "bots": bots.values().collect::<Vec<_>>(),
        "open_positions": positions.len(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}
