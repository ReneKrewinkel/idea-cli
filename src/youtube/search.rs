use rusty_ytdl::search::{SearchResult, YouTube};


fn strip_words(input: &String) -> String {

    let output =  input.replace(" in ", " ")
        .replace(" the ", " ")
        .replace(" from ", " ")
        .replace(" of ",  " ")
        .replace(" my ", " " )
        .replace(" our ", " " )
        .replace(", ", " " )
        .replace("i like ", " ")
        .replace("research ", " ")
        .replace("idea ", " ")
        .replace( "  ", " ").clone();

    output

}

pub(crate) async fn search_videos(search: String) -> Vec<String> {

    let youtube = YouTube::new().unwrap();
    let find = strip_words(&search.trim_matches('"').to_string());
    let res = youtube.search(&find, None).await;
    let mut urls = Vec::new();

    for result in res.unwrap() {
        match result {
            SearchResult::Video(v) => { urls.push( v.get_embed_url().unwrap().to_string() ) }
            _ => {}
        }
    }

    urls
}

