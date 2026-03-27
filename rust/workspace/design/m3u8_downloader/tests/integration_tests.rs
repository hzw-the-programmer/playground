use httpmock::prelude::*;
use reqwest::Client;

use m3u8_downloader::parse_m3u8;

#[tokio::test]
async fn test_parse_m3u8_relative() {
    let server = MockServer::start_async().await;
    let m3u8_content = r#"EXTM3U
#EXTINFO:10,
segment1.ts
#EXTINFO:10,
segment2.ts    
"#;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/playlist.m3u8");

            then.status(200).body(m3u8_content);
        })
        .await;

    let client = Client::new();
    let urls = parse_m3u8(&client, &server.url("/playlist.m3u8"))
        .await
        .unwrap();

    mock.assert();

    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], server.url("/segment1.ts"));
    assert_eq!(urls[1], server.url("/segment2.ts"));
}
