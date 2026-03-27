use std::sync::Arc;

use httpmock::prelude::*;
use reqwest::Client;
use tempfile::TempDir;
use tokio::sync::Semaphore;

use m3u8_downloader::{download_all_segments, download_segment, merge_segments, parse_m3u8};

fn mock_ts_data() -> Vec<u8> {
    vec![0x47, 0x40, 0x00, 0x10, 0x00]
}

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

#[tokio::test]
async fn test_parse_m3u8_absolute() {
    let server = MockServer::start_async().await;
    let m3u8_content = r#"#EXTM3U
#EXTINFO:10,
http://example.com/segment1.ts
#EXTINFO:10,
http://example.com/segment2.ts
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
    assert_eq!(urls[0], "http://example.com/segment1.ts");
    assert_eq!(urls[1], "http://example.com/segment2.ts");
}

#[tokio::test]
async fn test_download_segment() {
    let server = MockServer::start_async().await;
    let data = mock_ts_data();

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/segment1.ts");
            then.status(200).body(data.clone());
        })
        .await;

    let client = Client::new();
    let temp_dir = TempDir::new().unwrap();
    let semaphore = Arc::new(Semaphore::new(1));
    let path = download_segment(
        &client,
        &server.url("/segment1.ts"),
        temp_dir.path(),
        0,
        semaphore,
    )
    .await
    .unwrap();

    mock.assert();

    let content = tokio::fs::read(&path).await.unwrap();
    assert_eq!(content, data);
}

#[tokio::test]
async fn test_merge_segments() {
    let temp_dir = TempDir::new().unwrap();
    let seg1 = temp_dir.path().join("seg1.ts");
    let seg2 = temp_dir.path().join("seg2.ts");
    tokio::fs::write(&seg1, b"ABC").await.unwrap();
    tokio::fs::write(&seg2, b"def").await.unwrap();

    let output = temp_dir.path().join("output.ts");
    let segment_files = vec![seg1, seg2];
    merge_segments(&segment_files, &output).await.unwrap();

    let content = tokio::fs::read(output).await.unwrap();
    assert_eq!(content, b"ABCdef");
}

#[tokio::test]
async fn test_download_all_segments() {
    let server = MockServer::start_async().await;
    let data = mock_ts_data();

    let mock1 = server
        .mock_async(|when, then| {
            when.method(GET).path("/seg1.ts");
            then.status(200).body(data.clone());
        })
        .await;

    let mock2 = server
        .mock_async(|when, then| {
            when.method(GET).path("/seg2.ts");
            then.status(200).body(data.clone());
        })
        .await;

    let mock3 = server
        .mock_async(|when, then| {
            when.method(GET).path("/seg3.ts");
            then.status(200).body(data.clone());
        })
        .await;

    let urls = vec![
        server.url("/seg1.ts"),
        server.url("/seg2.ts"),
        server.url("/seg3.ts"),
    ];
    let client = Client::new();
    let (_temp_dir, segment_files) = download_all_segments(&client, urls, 2).await.unwrap();

    mock1.assert();
    mock2.assert();
    mock3.assert();

    assert_eq!(segment_files.len(), 3);
    for (i, path) in segment_files.iter().enumerate() {
        let content = tokio::fs::read(path).await.unwrap();
        assert_eq!(content, data);
        assert!(
            path.to_string_lossy()
                .contains(&format!("segment_{:05}.ts", i))
        );
    }
}
