use std::{env, future::Future};

use ollama_rs::{
    error::OllamaError,
    generation::completion::{request::GenerationRequest, GenerationResponse},
    Ollama,
};

#[derive(Copy, Clone)]
pub struct LlamaBot<'a> {
    pub model: &'a str,
}

pub trait Talk {
    fn talk(self, prompt: String) -> impl Future<Output = String>;

    fn get_response(
        self,
        ollama: Ollama,
        model: String,
        prompt: String,
    ) -> impl Future<Output = Result<GenerationResponse, OllamaError>>;
}

impl Talk for LlamaBot<'_> {
    async fn talk(self, prompt: String) -> String {
        let ollama = Ollama::new(
            env::var("LLM_URL").unwrap_or("http://localhost".to_owned()),
            11434,
        );
        let model = format!("{}:latest", self.model);
        let res = self.get_response(ollama, model, prompt).await;

        let full_resp = if let Ok(res) = res {
            res.response
        } else {
            String::new()
        };
        tracing::debug!("{} response: {}", self.model, full_resp);
        full_resp
    }

    async fn get_response(
        self,
        ollama: Ollama,
        model: String,
        prompt: String,
    ) -> Result<GenerationResponse, OllamaError> {
        ollama.generate(GenerationRequest::new(model, prompt)).await
    }
}
