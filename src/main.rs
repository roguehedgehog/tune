mod lyrics;
mod config;

extern crate clap;
extern crate tokio;

use clap::{App, Arg, SubCommand};
use lyrics::{Request, GeniusClient};
use config::{Config, configure};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let app = create_app();
    let args = app.get_matches();
    let cfg: Config = confy::load("tune")?;

    if args.is_present("configure") {
        return configure_app(cfg);
    }

    if let Some(video_args) = args.subcommand_matches("video") {
        return search_videos(
            cfg, 
            video_args.value_of("title").expect("title must be provided"),
        );
    }

    if let Some(lyric_args) = args.subcommand_matches("lyrics") {
        return search_lyrics(
            cfg,
            lyric_args.value_of("lyrics").expect("lyrics must be provided"),
            lyric_args.value_of("artist").unwrap_or(""),
        ).await;
    }

    Ok(())
}

fn create_app<'a, 'b>() -> App<'a, 'b>{
    App::new("Tune: song search by lyrics")
        .version("0.1.0")
        .about("https://github.com/wildpumpkin/tune")
        .subcommand(SubCommand::with_name("configure"))
        .subcommand(SubCommand::with_name("video")
            .arg(Arg::with_name("title")))
        .subcommand(SubCommand::with_name("lyrics")
            .arg(Arg::with_name("lyrics").index(1))
            .arg(Arg::with_name("artist").short("a").long("artist").takes_value(true))
        )
}

fn configure_app(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    confy::store("tune", configure(cfg))?;
    println!("Tune configuration has been saved.");

    Ok(())
}

fn search_videos(cfg: Config, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn search_lyrics(cfg: Config, lyrics: &str, artist: &str) -> Result<(), Box<dyn std::error::Error>> {
    let req = Request{
        lyrics: lyrics, 
        artist: artist,
    };

    let results = GeniusClient::with_config(cfg).search(req).await?;
    let hits = results.get_hits();

    println!("Found {} results", hits.len());
    for (i, hit) in hits.iter().enumerate() {
        println!("{}: {}", i, hit);
    }

    Ok(())
}

