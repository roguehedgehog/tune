use tune::config::Config;
use tune::{configure_app, create_app, search_lyrics, search_videos};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app();
    let args = app.get_matches();
    let cfg: Config = confy::load("tune")?;

    if args.is_present("configure") {
        configure_app(&cfg)?;
        println!("Tune configuration has been saved.");
    }

    if let Some(video_args) = args.subcommand_matches("video") {
        let videos = search_videos(
            &cfg,
            video_args
                .value_of("title")
                .expect("title must be provided"),
        )
        .await?;

        println!("Found {} results", videos.items.len());
        for video in videos.items {
            println!(
                "Title: {}\nDesription: {}\nLink: {}",
                video.get_title(),
                video.get_description(),
                video.get_location()
            )
        }
    }

    if let Some(lyric_args) = args.subcommand_matches("lyrics") {
        let songs = search_lyrics(
            &cfg,
            lyric_args
                .value_of("lyrics")
                .expect("lyrics must be provided"),
            lyric_args.value_of("artist").unwrap_or(""),
        )
        .await?
        .get_hits();

        println!("Found {} results", songs.len());
        for (i, hit) in songs.iter().enumerate() {
            println!("{}: {}", i, hit);
        }
    }

    Ok(())
}
