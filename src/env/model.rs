#[derive(Debug, Clone)]
pub struct Config {
    pub vault_path: String,
    pub openai_token: String,
    pub openai_url: String,
    pub openai_model: String,
    pub youtube_token: String,
}

impl Config {
    pub fn new(vault_path: String,
               openai_token: String,
               openai_url: String,
               openai_model: String,
               youtube_token: String) -> Config {
        Config { vault_path, openai_token, openai_url, openai_model, youtube_token }
    }
}