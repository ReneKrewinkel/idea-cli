use serde::{Deserialize, Serialize};
use crate::env::config::get_date;

#[derive(Debug)]
pub struct Note {
    pub file_name: String, 
    pub tags: Vec<String>,
    pub input_string: String,
    pub model: String, 
    pub completion: String, 
    pub search_criteria: String, 
    pub videos: Vec<String>
}


impl Note { 
    pub fn new(file_name: String,
               tags: Vec<String>,
               input_string: String,
               model: String,
               completion: String,
               search_criteria: String,
               videos: Vec<String>) -> Note {
        
        Note { file_name, tags, input_string,
               model, completion,
               search_criteria,
               videos 
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontMatter {
    title: String,
    tags: Vec<String>,
    handled: bool,
    model: String,
    created: String
}

impl FrontMatter {
    pub fn new(title: String, tags: Vec<String>, model: String) -> FrontMatter {
        FrontMatter {
            title,
            tags,
            model,
            handled: false,
            created:  get_date()
        }
    }
}