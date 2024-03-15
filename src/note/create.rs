use std::fs::File;
use std::io::Write;
use inflector::Inflector;
use crate::note::model::{FrontMatter, Note};

fn create_front_matter(input_string: &String, model: &String, tags: &mut Vec<String>) -> String {

    let mut tgs: Vec<String> = vec!["idea".to_string(), "inbox".to_string()];
    tgs.append(tags);
    let f = FrontMatter::new(input_string.clone(), tgs, model.clone());

    let yaml = serde_yaml::to_string(&f).unwrap();
    let front_matter = format!("---\n{yaml}---\n\n");
    
//  let front_matter = format!("---
// title: {}
// tags:
// - idea
// - inbox
// - {}
// {}
// handled: false
// model: {}
// created: {}
// ---\n\n", &input_string.to_sentence_case(), &model, &tags, &model, &date_string);

    front_matter.to_string()
}

fn create_video_embedding( url: &String) -> String {
    let embed = format!("<iframe width='840' height='480'
        src='{}'
        title='YouTube video player'
        frameborder='0'
        allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share'
        allowfullscreen>
</iframe>\n\n", &url);

    embed.to_string()
}

fn create_content(input_string: &String,
                  completion: &String, search_criteria: &String,
                  videos: &[String]) -> String {

    let txt_content = format!("### {}\n{}", &input_string.to_sentence_case(), &completion);
    let mut video_content = "".to_string();
    let query_string = &search_criteria.to_kebab_case().replace("-", "+");
    let youtube_query = format!("https://www.youtube.com/results?search_query={}", &query_string);

    for url in videos.iter().take(5) {
        let e = create_video_embedding(&url);
        video_content = format!("{}\n\n{}", video_content, e);
    }

    format!("\n{}\n\n### Videos\n**Query:** `{}`\n\n{}\n\n{}\n\n", &txt_content, &search_criteria, &youtube_query, &video_content)
}
fn write_note (file_name: &String, content: &String) -> bool {

    let mut file = match File::create(file_name) {
        Err(why) => panic!("Cannot create file: {}", why),
        Ok(file) => file
    };

    if let Err(why) = file.write_all(content.as_bytes()) { panic!("Cannot write to file: {}", why) };

    true
}
pub fn create_note(note: &Note) -> bool {

    let Note { file_name, 
               tags, 
               input_string, 
               model, 
               completion, 
               search_criteria, 
               videos } = &note;
    
    let mut m = tags.to_owned();

    let front_matter = create_front_matter(input_string, model, &mut m);
    let content = create_content(input_string, completion, search_criteria, videos);
    let note = format!("{} {}", &front_matter, &content);

    write_note(file_name, &note)

}
