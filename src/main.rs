mod search;

extern crate clap;

use clap::{App, Arg, SubCommand};
use search::request::Request;
use search::config::{Config, configure};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let app = App::new("Tune: song search by lyrics")
        .version("0.1.0")
        .about("https://github.com/wildpumpkin/tune")
        .arg(Arg::with_name("lyrics"))
        .arg(Arg::with_name("artist").short("a").long("artist").takes_value(true))
        .subcommand(SubCommand::with_name("configure"));

    let args = app.get_matches();
    let cfg: Config = confy::load("tune")?;

    if args.is_present("configure") {
        confy::store("tune", configure(cfg))?;
        println!("Tune configuration has been saved.");
        return Ok(());
    }

    let _req = Request{
        lyrics: args.value_of("lyrics").expect("lyric search string not provided."), 
        artist: args.value_of("artist").unwrap_or("")
    };

    Ok(())
}

