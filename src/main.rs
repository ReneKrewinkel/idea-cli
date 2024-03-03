extern crate chrono;
use inflector::cases::sentencecase::to_sentence_case;

mod openai;
use openai::completion::openai_completion;
mod ollama;
use ollama::completion::ollama_completion;

mod youtube;
use youtube::search;

mod note;
use note::create;
//use note::model::Note;
use crate::note::model::Note;
mod env;

use env::config::*;
use crate::env::model::Config;


async fn run_completion(prompt: &String, cfg: &Config) -> String {
    if &cfg.use_ollama == "YES" {
        ollama_completion(prompt, cfg).await
    } else {
        openai_completion(prompt, cfg).await
    }
}

async fn create_search_criteria(input: &String,  cfg: &Config) -> String {
     if cfg.use_ollama == "YES" {
        let search_prompt = format!("create a search argument for the following sentence '{}' no explanation just 1 sentence, omit  bullet points with a maximum of 5 words remove all unnecessary characters", &input);
        run_completion(&search_prompt, cfg).await
    } else {
        // Some weirdness when running openai with return characters (see #9)
        // work around: just use the input string
        input.clone()
    }
}

fn extract_model(cfg: &Config) -> String {
    if cfg.use_ollama == "YES" {
        cfg.ollama_model.clone()
    } else {
        cfg.openai_model.clone()
    }
}

#[tokio::main]
async fn main()  {

    let cfg = read_config();
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <string>", args[0]);
        return;
    }

    let input_string = &args[1];
    let prompt = format!("summarise '{}'", &input_string);
    let file_name = format!("{}/{}.md", cfg.vault_path, to_sentence_case(input_string));
    let completion = run_completion(&prompt, &cfg).await;
    let search_criteria= create_search_criteria(input_string, &cfg).await;
    let videos = search::search_videos(search_criteria.clone()).await;
    let model= extract_model(&cfg);
    let n = Note::new(file_name, input_string.clone(), model, completion, search_criteria, videos);
    let _result = create::create_note(&n);

}