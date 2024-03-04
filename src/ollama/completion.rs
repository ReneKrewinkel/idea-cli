use ollama_rs::{
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
    Ollama,
};
use crate::env::model::Config;

pub async fn ollama_completion(prompt: &String, cfg: &Config) -> String {

    let ollama = Ollama::default();

    let options = GenerationOptions::default()
        .temperature(0.2)
        .repeat_penalty(1.5)
        .top_k(25)
        .top_p(0.25);

    let res = ollama
        .generate(GenerationRequest::new(cfg.ollama_model.clone(), prompt.clone()).options(options))
        .await;

    match res {
        Ok(c) => { c.response.to_string() }
        Err(e) => { e }
    }

}