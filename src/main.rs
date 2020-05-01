use tune::config::Config;
use tune::{configure_app, create_app, search_lyrics, search_videos};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app();
    let args = app.get_matches();
    let cfg: Config = confy::load("tune")?;

    if args.is_present("configure") {
        return configure_app(cfg);
    }

    if let Some(video_args) = args.subcommand_matches("video") {
        return search_videos(
            cfg,
            video_args
                .value_of("title")
                .expect("title must be provided"),
        )
        .await;
    }

    if let Some(lyric_args) = args.subcommand_matches("lyrics") {
        return search_lyrics(
            cfg,
            lyric_args
                .value_of("lyrics")
                .expect("lyrics must be provided"),
            lyric_args.value_of("artist").unwrap_or(""),
        )
        .await;
    }

    Ok(())
}
