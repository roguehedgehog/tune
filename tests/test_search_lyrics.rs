extern crate tune;

use mockito::{mock, Matcher};
use tune::config::Config;
use tune::search_lyrics;

#[tokio::test]
async fn test_search_lyrics() {
    let genius = mock("GET", "/search")
        .match_header("authorization", "Bearer genius-token")
        .match_query(Matcher::Exact(format!(
            "q=Blondie+She+moves+like+she+don%27t+care"
        )))
        .with_body_from_file("tests/resources/genius-response.json")
        .create();

    let cfg = Config {
        genius_search_endpoint: format!("{}/search", mockito::server_url()),
        genius_api_key: "genius-token".into(),
        ..Default::default()
    };

    let tracks = search_lyrics(&cfg, "She moves like she don't care", "Blondie")
        .await
        .unwrap()
        .get_hits();

    genius.assert();
    assert_eq!(9, tracks.len());
    assert_eq!("Maria by Blondie", tracks[1])
}
