pub mod config;
mod http;
mod lyric;
mod video;

extern crate clap;
extern crate tokio;

use clap::{App, Arg, SubCommand};
use config::{configure, Config};
use lyric::GeniusClient;
use video::YouTubeClient;

pub fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Tune: song search by lyrics")
        .version("0.1.0")
        .about("https://github.com/wildpumpkin/tune")
        .subcommand(SubCommand::with_name("configure"))
        .subcommand(SubCommand::with_name("video").arg(Arg::with_name("title")))
        .subcommand(
            SubCommand::with_name("lyrics")
                .arg(Arg::with_name("lyrics").index(1))
                .arg(
                    Arg::with_name("artist")
                        .short("a")
                        .long("artist")
                        .takes_value(true),
                ),
        )
}

pub fn configure_app(cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
    confy::store("tune", configure(cfg))?;
    
    Ok(())
}

pub async fn search_videos(cfg: &Config, title: &str) -> Result<video::Response, Box<dyn std::error::Error>> {
    let req = video::Request {
        title,
        key: &cfg.youtube_api_key.to_owned(),
    };

    let client = YouTubeClient::with_config(cfg);

    Ok(client.search(req).await?)
}

pub async fn search_lyrics(
    cfg: &Config,
    lyrics: &str,
    artist: &str,
) -> Result<lyric::Results, Box<dyn std::error::Error>> {
    let req = lyric::Request { lyrics, artist };

    Ok(GeniusClient::with_config(cfg).search(req).await?)
}
