use mockito::{mock, Matcher};
use tune::config::Config;
use tune::search_videos;

#[tokio::test]
async fn test_search_videos() {
    // Given
    let youtube = mock("GET", "/youtube/v3/search")
        .match_query(Matcher::AnyOf(vec![
            Matcher::Regex("part=snippet".to_owned()),
            Matcher::Regex("q=Evil%27s+Sway+by+Japandroids".to_owned()),
            Matcher::Regex("type=video".to_owned()),
            Matcher::Regex("key=youtube-key".to_owned()),
        ]))
        .with_body_from_file("tests/resources/youtube-response.json")
        .create();

    let cfg = Config {
        youtube_search_endpoint: format!("{}/youtube/v3/search", mockito::server_url()),
        youtube_api_key: "youtube-key".to_owned(),
        ..Default::default()
    };

    // When
    let videos = search_videos(&cfg, "Evil's Sway by Japandroids").await;

    // Then
    youtube.assert();

    let videos = videos.unwrap();
    assert_eq!(5, videos.items.len());
    assert_eq!(
        "Japandroids - \"Evil´s Sway\"",
        videos.items[2].get_title()
    );
    assert_eq!(
        "Japandroids, München, Feierwerk, 11.09.12.",
        videos.items[2].get_description()
    );
    assert_eq!(
        "https://www.youtube.com/watch?v=1qhfS0AmMGM",
        videos.items[2].get_location()
    );
}
