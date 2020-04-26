extern crate serde;

use crate::http::Query;
pub struct Request<T> {
	title: T
}

impl Query for Request<&str> {
	fn to_string(&self) -> String {
		self.title.to_string()
	}
}

pub struct YouTubeClient {
	video_search_endpoint: String,
	api_key: String,
}

impl YouTubeClient {
	fn with_config(cfg: Config) -> Self {
		Self {
			video_search_endpoint: cfg.youtube_search_endpoint,
			api_key: cfg.youtube_api_key,
		}
	}

	fn search(&self, req: Request) {
		let params = Serializer::new()
	}
}