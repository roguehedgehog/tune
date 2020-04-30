extern crate reqwest;
extern crate url;

use crate::config::Config;
use crate::http::Query;
use reqwest::header;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
pub struct Request<T> {
    pub lyrics: T,
    pub artist: T,
}

impl Query for Request<&str> {
    fn get_params(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("q".to_string(), format!("{} {}", self.artist, self.lyrics));

        map
    }
}

pub struct GeniusClient {
    search_endpoint: String,
    api_key: String,
}

impl GeniusClient {
    pub fn with_config(cfg: Config) -> Self {
        Self {
            search_endpoint: cfg.genius_search_endpoint,
            api_key: cfg.genius_api_key,
        }
    }

    pub async fn search(&self, req: Request<&str>) -> Result<Results, Box<dyn Error>> {
        let url = req.get_url(self.search_endpoint.clone());
        let resp = self.create_client()?.get(&url).send().await?;
        let results: Results = serde_json::from_str(&resp.text().await?.to_owned())?;

        Ok(results)
    }

    fn create_client(&self) -> Result<Client, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", self.api_key).to_owned())?,
        );

        Ok(reqwest::Client::builder()
            .default_headers(headers)
            .build()?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Results {
    response: Response,
}
#[derive(Serialize, Deserialize)]
struct Response {
    hits: Vec<Hit>,
}
#[derive(Clone, Serialize, Deserialize)]
struct Hit {
    result: HitResult,
}
#[derive(Clone, Serialize, Deserialize)]
struct HitResult {
    full_title: String,
}

impl Results {
    pub fn get_hits(&self) -> Vec<String> {
        let mut hits = Vec::with_capacity(self.response.hits.len());
        for hit in &self.response.hits {
            hits.push(hit.result.full_title.clone())
        }

        hits
    }
}
