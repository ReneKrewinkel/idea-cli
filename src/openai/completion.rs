use openai::{completions::Completion, set_key};
use crate::env::model::Config;

pub async fn openai_completion(prompt: &String, cfg: &Config) -> String {

    set_key(cfg.openai_token.clone());
    let res = Completion::builder(&cfg.openai_model)
        .prompt(prompt)
        .max_tokens(1024)
        .create()
        .await;

    match res {
        Ok(c) => { c.choices.first().unwrap().text.clone().to_string() }
        Err(e) => { e.message.to_string() }
    }

}
