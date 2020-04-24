mod search;

extern crate clap;
extern crate tokio;

use clap::{App, Arg, SubCommand};
use search::request::{Request, GeniusClient};
use search::config::{Config, configure};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let app = App::new("Tune: song search by lyrics")
        .version("0.1.0")
        .about("https://github.com/wildpumpkin/tune")
        .subcommand(SubCommand::with_name("configure"))
        .subcommand(SubCommand::with_name("search")
            .arg(Arg::with_name("lyrics").index(1))
            .arg(Arg::with_name("artist").short("a").long("artist").takes_value(true))
        );

    let args = app.get_matches();
    let cfg: Config = confy::load("tune")?;

    if args.is_present("configure") {
        confy::store("tune", configure(cfg))?;
        println!("Tune configuration has been saved.");
        return Ok(());
    }

    if let Some(args) = args.subcommand_matches("search") {
        let req = Request{
            lyrics: args.value_of("lyrics").expect("lyric search string not provided."), 
            artist: args.value_of("artist").unwrap_or("")
        };

        let results = GeniusClient::with_config(cfg).search(req).await?;
        let hits = results.get_hits();

        println!("Found {} results", hits.len());
        for (i, hit) in hits.iter().enumerate() {
            println!("{}: {}", i, hit);
        }
    }

    Ok(())
}

