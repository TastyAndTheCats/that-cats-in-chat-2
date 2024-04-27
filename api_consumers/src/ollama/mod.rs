use std::{env, future::Future, pin::Pin};

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

    fn truncate_response_for_twitch(&self, resp: String) -> String {
        let resp = resp.split("\n").collect::<Vec<&str>>()[0].to_owned();
        let resp = if resp.len() > 450 {
            resp[..450].to_owned()
        } else {
            resp.to_owned()
        };
        resp
    }
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
        let short_resp = self.truncate_response_for_twitch(full_resp);
        tracing::debug!("{} response: {}", self.model, short_resp);
        short_resp
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
