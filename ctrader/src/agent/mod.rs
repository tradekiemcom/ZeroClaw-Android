use anyhow::Result;
use std::sync::Arc;
use crate::state::AppState;
use crate::models::OrderRequest;
use tracing::{info, error};

pub mod model;
pub mod prompter;

pub struct AiAgent {
    model: Option<model::QwenModel>,
}

impl AiAgent {
    pub fn new() -> Self {
        Self { model: None }
    }

    pub fn new_placeholder() -> Self {
        Self { model: None }
    }

    pub fn is_loaded(&self) -> bool {
        self.model.is_some()
    }

    pub async fn load_model(&mut self) -> Result<()> {
        if self.model.is_none() {
            info!("[AGENT] Loading Qwen-1.5B model...");
            self.model = Some(model::QwenModel::load().await?);
        }
        Ok(())
    }

    pub fn unload(&mut self) {
        self.model = None;
        info!("[AGENT] AI Agent model unloaded from memory.");
    }

    pub async fn process_natural_language(&mut self, text: &str, state: Arc<AppState>) -> Result<Option<OrderRequest>> {
        // 1. Check if loaded
        let model = match self.model.as_ref() {
            Some(m) => m,
            None => return Err(anyhow::anyhow!("AI Agent is currently OFF. Use '/agent on' to start.")),
        };

        info!("[AGENT] AI Agent processing: \"{}\"", text);
        
        // 2. Prepare system context (Balances, Positions)
        let context = prompter::prepare_context(&state).await;

        // 3. Inference
        let raw_output = match model.generate(text, &context).await {
            Ok(out) => out,
            Err(e) => {
                error!("[ERROR] AI Inference failed: {}", e);
                return Err(e);
            }
        };

        // 4. Parse structured JSON from AI output
        info!("[AGENT] AI Suggestion: {}", raw_output);
        prompter::parse_agent_json(&raw_output, &state).await
    }
}
