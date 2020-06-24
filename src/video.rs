extern crate htmlescape;
extern crate serde;
use crate::config::Config;
use crate::http::Query;
use htmlescape::decode_html;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
pub struct Request<'a> {
    pub title: &'a str,
    pub key: &'a str,
}

impl Query for Request<'_> {
    fn get_params(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("part".to_string(), "snippet".to_string());
        map.insert("type".to_owned(), "video".to_owned());
        map.insert("q".to_string(), self.title.to_string());
        map.insert("key".to_string(), self.key.to_string());

        map
    }
}

pub async fn search(req: Request<'_>, cfg: &Config) -> Result<Response, Box<dyn Error>> {
    let resp = reqwest::get(&req.get_url(&cfg.youtube_search_endpoint)).await?;
    let txt = resp.text().await?;
    let results: Response = serde_json::from_str(&txt)?;

    Ok(results)
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub items: Vec<Video>,
}

#[derive(Serialize, Deserialize)]
pub struct Video {
    id: VideoId,
    snippet: VideoInfo,
}

impl Video {
    pub fn get_title(&self) -> String {
        match decode_html(&self.snippet.title.to_owned()) {
            Ok(title) => title,
            Err(_) => self.snippet.title.clone(),
        }
    }

    pub fn get_description(&self) -> String {
        match decode_html(&self.snippet.description.to_owned()) {
            Ok(desc) => desc,
            Err(_) => self.snippet.description.clone(),
        }
    }

    pub fn get_location(&self) -> String {
        format!("https://www.youtube.com/watch?v={}", self.id.video_id)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoId {
    video_id: String,
}

#[derive(Serialize, Deserialize)]
struct VideoInfo {
    title: String,
    description: String,
}
