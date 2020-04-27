extern crate serde;
use std::collections::HashMap;
use std::error::Error;
use crate::http::Query;
use crate::config::Config;
pub struct Request<T> {
	pub title: T,
	pub key: T,
}

impl Query for Request<&str> {
	fn get_params(&self) -> HashMap<String, String> {
		let mut map = HashMap::new();
		map.insert("part".to_string(), "snippet".to_string());
		map.insert("q".to_string(), self.title.to_string());
		map.insert("key".to_string(), self.key.to_string());

		map
	}
}

pub struct YouTubeClient {
	video_search_endpoint: String,
}

impl YouTubeClient {
	pub fn with_config(cfg: Config) -> Self {
		Self {
			video_search_endpoint: cfg.youtube_search_endpoint,
		}
	}

	pub async fn search(&self, req: Request<&str>) -> Result<(), Box<dyn Error>>{
		let resp = reqwest::get(&req.get_url(self.video_search_endpoint.clone()).to_owned())
			.await?
			.text()
			.await?;

			println!("{:?}", resp);

			Ok(())
	}
}