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
use note::model::Note;
mod env;
use env::config::*;
use env::model::{Config, UseOllama};

async fn create_tags(input: &str, cfg: &Config) -> Vec<String> {
    let tag_prompt = format!("create obsidian tags for '{}' displayed on one line, display only the hashtags and without descriptive text, without an intro", input);
    let data = run_completion(&tag_prompt, cfg).await;

    if let Some(pos) = data.find('#') {
        let slice = &data[pos..];
        let v: Vec<String> = slice
            .split_whitespace()
            .filter(|word| word.starts_with('#'))
            .map(|word| word.to_string())
            .collect();
        v
    } else {
        Vec::new()
    }
}

async fn run_completion(prompt: &String, cfg: &Config) -> String {
    match &cfg.use_ollama {
        UseOllama::Yes => ollama_completion(prompt, cfg).await,
        UseOllama::No => openai_completion(prompt, cfg).await,
    }
}

async fn create_search_criteria(input: String, cfg: &Config) -> String {
    match &cfg.use_ollama {
        UseOllama::Yes => {
            let search_prompt = format!("create a search argument for the following sentence '{}' no explanation just 1 sentence, omit  bullet points with a maximum of 5 words remove all unnecessary characters", &input);
            run_completion(&search_prompt, cfg).await
        }
        UseOllama::No => input.clone(),
    }
}

fn extract_model(cfg: &Config) -> String {
    match &cfg.use_ollama {
        UseOllama::Yes => cfg.ollama_model.clone(),
        UseOllama::No => cfg.openai_model.clone(),
    }
}

#[tokio::main]
async fn main() {
    let cfg = read_config();
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} 'sentence with information to research'", args[0]);
        return;
    }

    let input_string = &args[1];
    let prompt = format!("summarise '{}'", &input_string);
    // Find new thing for uniqueness
    let file_name = format!(
        "{}/{}-({}).md",
        cfg.vault_path,
        to_sentence_case(input_string),
        get_date()
    );
    let completion = run_completion(&prompt, &cfg).await;
    let search_criteria = create_search_criteria(input_string.to_string(), &cfg).await;
    let tags = create_tags(input_string, &cfg).await;
    let videos = search::search_videos(prompt.clone()).await;
    let model = extract_model(&cfg);
    let n = Note::new(
        file_name.clone(),
        tags,
        input_string.to_string(),
        model,
        completion,
        search_criteria,
        videos,
    );
    let _ = create::create_note(&n);

    println!("{}", &file_name);
}
