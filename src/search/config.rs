use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
	genius_api_key: String,
	youtube_api_key: String,
}

impl ::std::default::Default for Config {
	fn default() -> Self {
		Self {
			genius_api_key: String::new(),
			youtube_api_key: String::new(),
		}
	}
}

pub fn configure(cfg: Config) -> Config {
	let mut new_genius_key = String::new();
	let mut new_youtube_key: String = String::new();

	println!("Enter your Genius Client Access Token: [{}]", cfg.genius_api_key);
	io::stdin().read_line(&mut new_genius_key).expect("Error reading Genius Token");

	println!("Enter your YouTube API key: [{}]", cfg.youtube_api_key);
	io::stdin().read_line(&mut new_youtube_key).expect("Error reading YouTube Key");

	Config {
		genius_api_key: if new_genius_key.trim() == "" { cfg.genius_api_key } else { new_genius_key.trim().to_string() },
		youtube_api_key: if new_youtube_key.trim() == "" { cfg.youtube_api_key } else { new_youtube_key.trim().to_string() },
	}
}