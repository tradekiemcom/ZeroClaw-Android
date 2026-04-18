# iZ-Trade (ZeroClaw Hub) Technical Guide & Roadmap

## 1. Project Mission
**iZ-Trade** is a high-performance, multi-account trading hub designed to synchronize trading operations across multiple cTrader accounts. It provides a unified interface via **Telegram Bot** and **CLI (iZ-Console)**, supporting manual execution, automated grid trading, and real-time monitoring.

---

## 2. Core Architecture

### 2.1 State Management (`src/state.rs`)
The `AppState` is the "Source of Truth" for the entire application. It is wrapped in an `Arc<AppState>` for thread-safe access across the CLI, Telegram Bot, and Background Workers.
- **Account Map**: In-memory storage of all connected trading accounts.
- **API Key Registry**: Manages permissions for external integrations.
- **Real-time Metrics**: Tracks total balance, equity, and P&L across all accounts.

### 2.2 Connection Management (`src/ctrader/`)
The system follows a **Pool-Session** architecture:
- **`ConnectionPool`**: Manages a fleet of `AccountSession` instances.
- **`AccountSession`**: A dedicated asynchronous task for each account that handles:
    - TLS TCP connection to cTrader OpenAPI.
    - Application & Account Authentication.
    - Heartbeat (PING/PONG) to maintain connection.
    - Event Listening: Captures live price feeds and execution events.

### 2.3 Execution Engine (`src/engine/`)
- **`IzParser`**: Translates human-readable strings (e.g., `#BUY XAUUSD 0.01`) into structured `OrderRequest`.
- **`Dispatch`**: Routes the `OrderRequest` to the correct account(s) via the `ConnectionPool`.

---

## 3. Module Map

| Path | Description |
| :--- | :--- |
| `src/main.rs` | Entry point. Initializes DB, State, CLI, and Teloxide Bot. |
| `src/cli.rs` | iZ-Console implementation (Interactive CLI). |
| `src/telegram/` | Telegram Bot handlers and session management. |
| `src/ctrader/` | cTrader OpenAPI implementation (Pool, Session, Proto). |
| `src/storage/` | SQLite / SQLx integration for persistence. |
| `src/models/` | Shared data structures (Order, Account, ApiClient). |
| `src/api/` | REST API routes for external web/app connections. |

---

## 4. Trading Commands (Standardized)

The system uses a unified command format prefix with `#`:
- `#BUY / #SELL`: Market order.
- `#BUYLIMIT / #SELLLIMIT`: Pending limit order.
- `#CLOSE / #MODIFY / #DELETE`: Management actions.

**Advanced Grid Parameters (Optional):**
- `Grib`: Number of grid levels.
- `Step`: Distance between levels (in points).
- `Xlot`: Volume multiplier for subsequent orders.
- `Life`: Expiration time in minutes.

*Example:* `#BUYLIMIT XAUUSD 0.01 Open: 2000 Grib: 5 Step: 50 Xlot: 1.5`

---

## 5. Current Development State (As of April 13, 2026)

### [DONE] Completed
- **Storage Layer:** Robust SQLite implementation with successful migrations.
- **CLI v2.1.0:** Superior Command Line Interface with dedicated modules:
    - `acc`: Full account lifecycle (add, del, list, toggle autotrade, status).
    - `api`: External API Key management (list, add, generate, toggle, delete).
    - `sys`: Advanced system maintenance (cleanup, nuclear wipe, log tailing).
- **Compilation & Logic Fixes:** Resolved all critical type mismatches and missing method calls in `pool.rs`, `session.rs`, and `state.rs`.
- **State Sync:** Real-time synchronization of balance/equity from DB to memory.

### 🚧 In Progress
- **Real-time Grid Execution:** Finalizing the logic to spawn multiple child orders (Engine v2).
- **Live WebSocket Integration:** Completing the Protobuf heartbeat and listener loop for live cTrader connections.
- **Cross-Platform Readiness:** specialized environment setups prepared for Mac M1 and Termux (Galaxy Note 10+).

---

## 6. Deployment & Testing (Mock Mode)

To test the system locally without real cTrader credentials:
1. Ensure `.env` is configured with `CTRADER_MODE=mock`.
2. Run `cargo run`.
3. Use the CLI: `iz > l` to see accounts, then `iz > 101` to enter account scope.
4. Execute trades: `iz[101] > #BUY XAUUSD 0.1` to see mock execution events.

---

## 7. Handover Protocol for Future Agents

1. **Context First:** Always read `SYSTEM_GUIDE.md`, `AuditReport.md`, and `implementation_plan.md` before making changes.
2. **Safety:** Do NOT modify `src/storage/db.rs` unless adding new migrations. The Borrow Checker logic here is critical for `sqlx` stability.
3. **Async Rules:** Use `tokio::spawn` for long-running network tasks. Ensure all `AccountSession` messages are routed through the `ConnectionPool`.
4. **Consistency:** Ensure UI changes are applied to BOTH `cli.rs` and `telegram/bot.rs` for feature parity.
5. **Clean Slate:** During heavy development, use `sys nuclear` in CLI to reset the database and test from scratch.
