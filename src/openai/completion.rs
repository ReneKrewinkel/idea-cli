use openai::{completions::Completion, set_key};
use crate::env::model::Config;

pub async fn openai_completion(prompt: &String, cfg: &Config) -> String {

    set_key(cfg.openai_token.clone());
    let res = Completion::builder(&cfg.openai_model)
        .prompt(prompt.clone())
        .max_tokens(1024)
        .create()
        .await;

    if let Ok(res) = res {
        return res.choices.first().unwrap().text.clone().to_string();
    }

    // let response = &res.choices.first().unwrap().text.clone();
    "".to_string()
}
