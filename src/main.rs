mod search;

extern crate clap;

use clap::{App, Arg};
use search::request::{Request};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let app = App::new("Tune: song search by lyrics")
        .version("0.1.0")
        .about("https://github.com/wildpumpkin/tune")
        .arg(Arg::with_name("lyrics").required(true))
        .arg(Arg::with_name("artist").short("a").long("artist").takes_value(true))
        .arg(Arg::with_name("config").short("c").long("config").takes_value(true));

    let args = app.get_matches();
    let _req = Request{
        lyrics: args.value_of("lyrics").expect("cannot get lyric search string"), 
        artist: args.value_of("artist").unwrap_or("")
    };

    Ok(())
}
