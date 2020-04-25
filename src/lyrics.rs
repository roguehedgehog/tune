extern crate reqwest;
extern crate url;

use reqwest::header;
use url::form_urlencoded::Serializer;
use std::error::Error;
use crate::config::Config;
use serde::{Serialize, Deserialize};
use serde_json;
pub struct Request<T> {
	pub lyrics: T,
	pub artist: T
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Results {
	response: Response
}
#[derive(Debug, Serialize, Deserialize)]
struct Response {
	hits: Vec<Hit>
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Hit {
 result: HitResult
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct HitResult {
 full_title: String
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
		let params = Serializer::new(String::new())
					.append_pair("q", &format!("{} {}", req.artist, req.lyrics).to_owned())
					.finish();
		let url = format!("{}?{}", &self.search_endpoint, &params).to_owned();
		let mut headers = header::HeaderMap::new();
		headers.insert(
			header::AUTHORIZATION, 
			header::HeaderValue::from_str(&format!("Bearer {}", self.api_key).to_owned())?
		);
		let http = reqwest::Client::builder()
			.default_headers(headers)
			.build()?;
		let resp = http.get(&url).send().await?;
		let text = resp.text().await?;
		let results: Results = serde_json::from_str(&text)?;

		Ok(results)
	}
}