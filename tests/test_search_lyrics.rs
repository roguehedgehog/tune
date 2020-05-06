extern crate tune;

use mockito::{mock, Matcher};
use tune::config::Config;
use tune::search_lyrics;

#[tokio::test]
async fn test_search_lyrics() {
    // Given
    let genius = mock("GET", "/search")
        .match_header("authorization", "Bearer genius-token")
        .match_query(Matcher::from("q=Blondie+She+moves+like+she+don%27t+care"))
        .with_body_from_file("tests/resources/genius-response.json")
        .create();

    let cfg = Config {
        genius_search_endpoint: format!("{}/search", mockito::server_url()),
        genius_api_key: "genius-token".into(),
        ..Default::default()
    };

    // When
    let tracks = search_lyrics(&cfg, "She moves like she don't care", "Blondie").await;

    // Then
    genius.assert();
    let tracks = tracks.unwrap().get_hits();
    assert_eq!(9, tracks.len());
    assert_eq!("Maria by Blondie", tracks[1])
}
