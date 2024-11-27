use ollama_rs::{
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
    Ollama,
};

use crate::env::model::Config;

pub async fn ollama_completion(prompt: &String, cfg: &Config) -> String {

    let options = GenerationOptions::default()
        .temperature(0.2)
        .repeat_penalty(1.5)
        .top_k(25)
        .top_p(0.25);

    match Ollama::default()
        .generate(GenerationRequest::new(cfg.ollama_model.to_string(), prompt.to_string()).options(options))
        .await {
            Ok(c) => { c.response.to_string() }
            Err(e) => { e.to_string() }
    }

}
