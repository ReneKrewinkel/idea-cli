#[derive(Debug, Clone)]
pub struct Config {
    pub vault_path: String,
    pub openai_token: String,
    pub openai_url: String,
    pub openai_model: String,
    // Let's default this
    //pub ollama_url: String,
    pub use_ollama: UseOllama,
    pub ollama_model: String,
    pub youtube_token: String,
}
#[derive(Debug, Clone)]
pub enum UseOllama {
    Yes,
    No
}
impl Config {
    pub fn new(vault_path: String,
               openai_token: String,
               openai_url: String,
               openai_model: String,
               ollama: String,
               ollama_model: String,
               youtube_token: String) -> Config {
        
        let use_ollama = match ollama.trim().to_uppercase().as_str() {
            "YES" | "Y" | "TRUE" => UseOllama::Yes,
            _ => UseOllama::No,
        };
        
        Config { vault_path,
                 openai_token, openai_url, openai_model,
                 use_ollama, ollama_model,
                 youtube_token }
    }
}