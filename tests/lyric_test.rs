extern crate tune;

use mockito::{mock, Matcher};
use tune::config::Config;
use tune::lyric::{GeniusClient, Request};

#[tokio::test]
async fn make_lyric_request() {
    let genius = mock("GET", "/search")
        .match_header("authorization", "Bearer genius-token")
        .match_query(Matcher::Exact(format!(
            "q=Blondie+She+moves+like+she+don%27t+care"
        )))
        .with_status(201)
        .with_header("content-type", "text/plain")
        .with_header("x-api-key", "1234")
        .with_body("world")
        .with_body_from_file("tests/resources/genius-response.json")
        .create();

    let cfg = Config {
        genius_search_endpoint: format!("{}/search", mockito::server_url()),
        genius_api_key: "genius-token".into(),
        ..Default::default()
    };

    let tracks = GeniusClient::with_config(&cfg)
        .search(Request {
            lyrics: "She moves like she don't care",
            artist: "Blondie",
        })
        .await
        .unwrap()
        .get_hits();

    genius.assert();
    assert_eq!(9, tracks.len());
    assert_eq!("Maria by Blondie", tracks[1])
}
