use anyhow::{Result, anyhow};
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights;
use candle_transformers::generation::LogitsProcessor;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;
use std::path::PathBuf;
use tracing::info;

pub struct QwenModel {
    weights: ModelWeights,
    tokenizer: Tokenizer,
    device: Device,
}

impl QwenModel {
    pub async fn load() -> Result<Self> {
        let device = Device::Cpu; // Default to CPU for ARM/Mobile compatibility

        // 1. Download Model & Tokenizer
        let (model_path, tokenizer_path) = tokio::task::spawn_blocking(move || {
            let api = Api::new()?;
            let repo = api.repo(Repo::with_revision(
                "Qwen/Qwen2.5-1.5B-Instruct-GGUF".to_string(),
                RepoType::Model,
                "main".to_string(),
            ));

            let model_file = repo.get("qwen2.5-1.5b-instruct-q4_k_m.gguf")?;
            
            // For tokenizer, we use the non-quantized repo's json
            let tokenizer_repo = api.repo(Repo::with_revision(
                "Qwen/Qwen2.5-1.5B-Instruct".to_string(),
                RepoType::Model,
                "main".to_string(),
            ));
            let tokenizer_file = tokenizer_repo.get("tokenizer.json")?;

            Ok::<(PathBuf, PathBuf), anyhow::Error>((model_file, tokenizer_file))
        }).await??;

        // 2. Initialize Tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow!("Tokenizer init failed: {}", e))?;

        // 3. Load GGUF Weights
        let mut file = std::fs::File::open(&model_path)?;
        let gguf = candle_core::quantized::gguf_file::Content::read(&mut file)?;
        let weights = ModelWeights::from_gguf(gguf, &mut file, &device)
            .map_err(|e| anyhow!("ModelWeights init failed: {}", e))?;

        Ok(Self {
            weights,
            tokenizer,
            device,
        })
    }

    pub async fn generate(&self, prompt: &str, system_context: &str) -> Result<String> {
        let full_prompt = format!(
            "<|im_start|>system\n{}<|im_end|>\n<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n",
            system_context, prompt
        );

        let tokens = self.tokenizer.encode(full_prompt, true)
            .map_err(|e| anyhow!("Encoding failed: {}", e))?;
        let tokens_ids = tokens.get_ids().to_vec();

        // ── Heavy Inference Loop Offloaded ─────────────────────────────
        // We move the sync model logic to a blocking thread to avoid starvation
        let weights = self.weights.clone();
        let tokenizer = self.tokenizer.clone();
        let device = self.device.clone();

        tokio::task::spawn_blocking(move || {
            let mut tokens = tokens_ids;
            let mut logits_processor = LogitsProcessor::new(1337, Some(0.0), None);
            let mut output_text = String::new();
            let eos_token = tokenizer.token_to_id("<|im_end|>").unwrap_or(151645);

            for i in 0..512 {
                let context_size = if i > 0 { 1 } else { tokens.len() };
                let start_pos = tokens.len().saturating_sub(context_size);
                let input = Tensor::new(&tokens[start_pos..], &device)?.unsqueeze(0)?;
                
                let logits = weights.clone().forward(&input, start_pos)?;
                let logits = logits.squeeze(0)?;
                let logits = logits.get(logits.dim(0)? - 1)?;

                let next_token = logits_processor.sample(&logits)?;
                tokens.push(next_token);

                if next_token == eos_token {
                    break;
                }

                if let Ok(decoded) = tokenizer.decode(&[next_token], true) {
                    output_text.push_str(&decoded);
                }

                if output_text.contains("<|im_end|>") {
                    break;
                }
            }
            Ok(output_text.replace("<|im_end|>", "").trim().to_string())
        }).await?
    }
}
