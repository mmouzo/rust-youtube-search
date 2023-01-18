use gloo_net::{http::Request, Error};
use serde::Deserialize;

use crate::api::API_KEY;

pub async fn search_youtube(text: String) -> Result<VideoItem, Error> {
    let query_url: String = format!(
        "https://www.googleapis.com/youtube/v3/search?part=id%2Csnippet&q={}",
        text
    );

    let mut auth: String = String::from("Bearer");
    auth.push_str(API_KEY);
    let response = Request::get(&query_url)
        .header("Authorization", &auth)
        .send()
        .await?;

    let search_result: SearchResult = response.json::<SearchResult>().await?;
    let empty_video = build_empty_video();
    let video: &VideoItem = match search_result.items.first() {
        Some(video) => video,
        None => &empty_video,
    };
    web_sys::console::log_1(&text.into());

    Ok(video.clone())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    items: Vec<VideoItem>,
}

#[derive(Clone, Deserialize)]
pub struct VideoItem {
    pub id: VideoItemId,
    pub snippet: VideoSnippet,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoItemId {
    pub kind: String,
    pub video_id: String,
}

#[derive(Clone, Deserialize)]
pub struct VideoSnippet {
    pub title: String,
    pub description: String,
}

fn build_empty_video() -> VideoItem {
    VideoItem {
        id: VideoItemId {
            kind: "".to_string(),
            video_id: "".to_string(),
        },
        snippet: VideoSnippet {
            title: "".to_string(),
            description: "".to_string(),
        },
    }
}
