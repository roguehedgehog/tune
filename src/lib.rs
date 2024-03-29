pub mod config;
mod http;
mod lyric;
mod video;

#[macro_use]
extern crate clap;
extern crate tokio;

use clap::{App, Arg, SubCommand};
use config::{configure, Config};

pub fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
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

pub async fn search_videos(
    cfg: &Config,
    title: &str,
) -> Result<video::Response, Box<dyn std::error::Error>> {
    let req = video::Request {
        title,
        key: &cfg.youtube_api_key,
    };

    Ok(video::search(req, cfg).await?)
}

pub async fn search_lyrics(
    cfg: &Config,
    lyrics: &str,
    artist: &str,
) -> Result<lyric::Results, Box<dyn std::error::Error>> {
    let req = lyric::Request { lyrics, artist };

    Ok(lyric::search(req, cfg).await?)
}
