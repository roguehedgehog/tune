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

        return Ok(());
    }

    if let Some(video_args) = args.subcommand_matches("video") {
        return video(
            &cfg,
            video_args
                .value_of("title")
                .expect("title must be provided"),
        )
        .await;
    }

    if let Some(lyric_args) = args.subcommand_matches("lyrics") {
        return lyrics(
            &cfg,
            lyric_args
                .value_of("lyrics")
                .expect("lyrics must be provided"),
            lyric_args.value_of("artist").unwrap_or(""),
        )
        .await;
    }

    Ok(())
}

async fn lyrics(
    cfg: &Config,
    lyrics: &str,
    artist: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let songs = search_lyrics(cfg, lyrics, artist).await?.get_hits();

    if songs.len() == 0 {
        println!("No songs could be found");
        return Ok(());
    }

    println!("Found {} results", songs.len());
    for (i, hit) in songs.iter().enumerate() {
        println!("{}: {}", i, hit);
    }

    loop {
        let mut selection = String::new();
        println!("Enter the number of your selection to search videos (q to quit): ");
        std::io::stdin().read_line(&mut selection)?;
        selection = selection.trim().to_string();
        if &selection[..] == "q" {
            break Ok(());
        }

        let sel: usize = selection.parse()?;
        match songs.get(sel) {
            Some(title) => {
                video(&cfg, title).await?;
                break Ok(());
            }
            None => println!("Invalid selection {}", selection),
        }
    }
}

async fn video(cfg: &Config, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let videos = search_videos(cfg, title).await?;
    println!("Found {} results\n", videos.items.len());
    for video in videos.items {
        println!(
            "Title: {}\n Desc: {}\nWatch: {}\n",
            video.get_title(),
            video.get_description(),
            video.get_location()
        )
    }

    Ok(())
}
