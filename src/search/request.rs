extern crate reqwest;
extern crate url;

use reqwest::header;
use url::form_urlencoded::Serializer;
use std::error::Error;
use crate::search::config::Config;
pub struct Request<T> {
	pub lyrics: T,
	pub artist: T
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

	pub async fn search(&self, req: Request<&str>) -> Result<(), Box<dyn Error>> {
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

		println!("{:?}", resp.text().await);

		Ok(())
	}
}