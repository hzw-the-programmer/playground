use httpmock::prelude::*;
use m3u8_downloader::{download_all_segments, merge_segments, parse_m3u8};
use reqwest::Client;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// 辅助函数：创建模拟的 TS 数据
fn mock_ts_data() -> Vec<u8> {
    vec![0x47, 0x40, 0x00, 0x10, 0x00] // 简化的 TS 包头
}

/// 测试解析 M3U8（相对路径）
#[tokio::test]
async fn test_parse_m3u8_relative() {
    let server = MockServer::start();
    let m3u8_content = r#"#EXTM3U
#EXTINF:10,
segment1.ts
#EXTINF:10,
segment2.ts
"#;
    let mock = server.mock(|when, then| {
        when.method(GET).path("/playlist.m3u8");
        then.status(200).body(m3u8_content);
    });

    let client = Client::new();
    let playlist_url = server.url("/playlist.m3u8");
    let urls = parse_m3u8(&client, &playlist_url).await.unwrap();

    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], server.url("/segment1.ts"));
    assert_eq!(urls[1], server.url("/segment2.ts"));

    mock.assert();
}

/// 测试解析 M3U8（绝对路径）
#[tokio::test]
async fn test_parse_m3u8_absolute() {
    let server = MockServer::start();
    let m3u8_content = r#"#EXTM3U
#EXTINF:10,
http://example.com/segment1.ts
#EXTINF:10,
http://example.com/segment2.ts
"#;
    let mock = server.mock(|when, then| {
        when.method(GET).path("/playlist.m3u8");
        then.status(200).body(m3u8_content);
    });

    let client = Client::new();
    let playlist_url = server.url("/playlist.m3u8");
    let urls = parse_m3u8(&client, &playlist_url).await.unwrap();

    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "http://example.com/segment1.ts");
    assert_eq!(urls[1], "http://example.com/segment2.ts");

    mock.assert();
}

/// 测试下载单个片段
#[tokio::test]
async fn test_download_segment() {
    let server = MockServer::start();
    let data = mock_ts_data();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/segment1.ts");
        then.status(200).body(data.clone());
    });

    let client = Client::new();
    let temp_dir = TempDir::new().unwrap();
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(1));
    let path = m3u8_downloader::download_segment(
        &client,
        &server.url("/segment1.ts"),
        temp_dir.path(),
        0,
        semaphore,
    )
    .await
    .unwrap();

    let content = fs::read(&path).unwrap();
    assert_eq!(content, data);
    mock.assert();
}

/// 测试合并片段
#[test]
fn test_merge_segments() {
    let temp_dir = TempDir::new().unwrap();
    let seg1 = temp_dir.path().join("seg1.ts");
    let seg2 = temp_dir.path().join("seg2.ts");
    fs::write(&seg1, b"ABC").unwrap();
    fs::write(&seg2, b"DEF").unwrap();

    let output = temp_dir.path().join("output.ts");
    let segment_files = vec![seg1, seg2];
    merge_segments(&segment_files, &output).unwrap();

    let content = fs::read(output).unwrap();
    assert_eq!(content, b"ABCDEF");
}

/// 测试完整下载流程（模拟 3 个片段）
#[tokio::test]
async fn test_download_all_segments() {
    let server = MockServer::start();
    let data = mock_ts_data();

    // 模拟 3 个 TS 片段
    let mock1 = server.mock(|when, then| {
        when.method(GET).path("/seg0.ts");
        then.status(200).body(data.clone());
    });
    let mock2 = server.mock(|when, then| {
        when.method(GET).path("/seg1.ts");
        then.status(200).body(data.clone());
    });
    let mock3 = server.mock(|when, then| {
        when.method(GET).path("/seg2.ts");
        then.status(200).body(data.clone());
    });

    let urls = vec![
        server.url("/seg0.ts"),
        server.url("/seg1.ts"),
        server.url("/seg2.ts"),
    ];
    let client = Client::new();
    let (temp_dir, segment_files) = download_all_segments(&client, urls, 2).await.unwrap();

    assert_eq!(segment_files.len(), 3);
    for (i, path) in segment_files.iter().enumerate() {
        let content = fs::read(path).unwrap();
        assert_eq!(content, data);
        assert!(path.to_string_lossy().contains(&format!("segment_{:05}.ts", i)));
    }

    mock1.assert();
    mock2.assert();
    mock3.assert();
    // temp_dir 自动清理
}