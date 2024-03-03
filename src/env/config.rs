//use env_file_reader::read_file;
use dotenv::{ dotenv };
// use homedir::get_my_home;
// use std::{ format };
use super::model::Config;

pub fn read_config() -> Config {

   // let binding = get_my_home().unwrap().expect("Homedir missing").to_path_buf();
   // let home = binding.to_str();
   // let file = format!("{}/.env/idea/env.env", &home.unwrap());

    dotenv().ok();

    let vault_path = std::env::var("VAULT_PATH").expect("VAULT_PATH must be set.");
    let openai_token = std::env::var("OPENAI_TOKEN").expect("OPENAI_TOKEN must be set.");
    let openai_url = std::env::var("OPENAI_URL").expect("OPENAI_URL must be set.");
    let openai_model =  std::env::var("OPENAI_MODEL").expect("OPENAI_MODEL must be set.");
    let ollama_model = std::env::var("OLLAMA_MODEL").expect("OLLAMA_MODEL must be set.");
    let use_ollama = std::env::var("USE_OLLAMA").expect("USE_OLLAMA must be set to YES or NO");
    let youtube_token = std::env::var("YOUTUBE_TOKEN").expect("YOUTUBE_TOKEN must be set.");

    Config::new(vault_path.clone(),
                openai_token.clone(),
                openai_url.clone(),
                openai_model.clone(),
                use_ollama.to_uppercase().clone(),
                ollama_model.clone(),
                youtube_token.clone(),
    )


}