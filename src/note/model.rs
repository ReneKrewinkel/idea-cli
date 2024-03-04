#[derive(Debug)]
pub struct Note {
    pub file_name: String, 
    pub tags: String,
    pub input_string: String,
    pub model: String, 
    pub completion: String, 
    pub search_criteria: String, 
    pub videos: Vec<String>
}


impl Note { 
    pub fn new(file_name: String,
               tags: String, 
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