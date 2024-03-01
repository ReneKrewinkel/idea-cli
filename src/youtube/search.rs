use rusty_ytdl::search::{SearchResult, YouTube};


fn strip_words(input: String) -> String {

    input.replace(" in ", " ")
        .replace(" the ", " ")
        .replace(" from ", " ")
        .replace(" of ",  " ")
        .replace(" my ", " " )
        .replace(" our ", " " )
        .replace(", ", " " )
        .replace("research ", " ")
        .replace("idea ", " ")
        .replace( "  ", " ")

}

pub(crate) async fn search_videos(search: String) -> Vec<String> {

    let youtube = YouTube::new().unwrap();
    let res = youtube.search(strip_words(search.as_str().trim_matches('"').to_string()), None).await;
    let mut urls = Vec::new();

    for result in res.unwrap() {
        match result {
            SearchResult::Video(v) => { urls.push( v.get_embed_url().unwrap().to_string() ) }
            _ => {}
        }
    }

    urls
}

