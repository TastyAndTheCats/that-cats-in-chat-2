use std::env;

use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

pub async fn talk_to_chatbot(model: &str, prompt: String) -> String {
    let ollama = Ollama::new(
        env::var("LLM_URL").unwrap_or("http://localhost".to_owned()),
        11434,
    );
    let model = format!("{}:latest", model);
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
        res.response
    } else {
        String::new()
    }
}
