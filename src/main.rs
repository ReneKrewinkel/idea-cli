use std;

extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{ SystemTime };
use inflector::cases::sentencecase::to_sentence_case;

mod openai;
use openai::completion::openai_completion;
mod youtube;
use youtube::search;

mod note;
use note::create;
mod env;
mod ollama;
use ollama::completion::ollama_completion;

use env::config::*;
use crate::env::model::Config;

fn get_date() -> String {
    let d = SystemTime::now();
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d").to_string();
    timestamp_str
}

async fn run_completion(prompt: &String, cfg: &Config) -> String {

    let completion: String;
    if &cfg.use_ollama == "YES" {
        completion = ollama_completion(&prompt, &cfg).await;
    } else {
        completion = openai_completion(&prompt, &cfg).await;
    }

    completion
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

    // Generate prompts
    let prompt = format!("summarise '{}'", &input_string);

    // Date & File name
    let date_string = get_date();
    let file_name = format!("{}/{}.md", cfg.vault_path, to_sentence_case(&input_string));

    // Execute completion
    let completion = run_completion(&prompt, &cfg).await;

    let model= if cfg.use_ollama == "YES" {
        cfg.ollama_model.clone()
    } else {
        cfg.openai_model.clone()
    };

    let search_criteria: String = if cfg.use_ollama == "YES" {
        let search_prompt = format!("create a search argument for the following sentence '{}' no explanation just 1 sentence, omit  bullet points with a maximum of 5 words remove all unnecessary characters", &input_string);
        run_completion(&search_prompt, &cfg).await
    } else {
        // Some weirdness when running openai with return characters (see #9)
        // work around: just use the input string
        input_string.clone()
    };

    let videos = search::search_videos(search_criteria.clone()).await;

    let _result = create::create_note(&file_name, &input_string,&model, &date_string, &completion, &search_criteria, &videos);


}