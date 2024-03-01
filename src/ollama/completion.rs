use ollama_rs::{
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
    Ollama,
};
use crate::env::model::Config;

pub async fn ollama_completion(prompt: &String, cfg: &Config) -> String {

    let ollama = Ollama::default();
    let model = cfg.ollama_model.to_string();
    let prompt = prompt.to_string();

    let options = GenerationOptions::default()
        .temperature(0.2)
        .repeat_penalty(1.5)
        .top_k(25)
        .top_p(0.25);

    let res = ollama
        .generate(GenerationRequest::new(model, prompt).options(options))
        .await;

    if let Ok(res) = res {
        return format!("{}", res.response);
    }

    "".to_string()
}