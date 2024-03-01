use openai::{completions::Completion, set_key};
use crate::env::model::Config;

pub async fn openai_completion(prompt: &String, cfg: &Config) -> String {

    set_key(cfg.openai_token.clone());
    let completion = Completion::builder(&*cfg.openai_model)
        .prompt(&prompt.clone())
        .max_tokens(1024)
        .create()
        .await
        .unwrap();

    let response = &completion.choices.first().unwrap().text.clone();
    response.to_string()
}
