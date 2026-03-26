use anyhow::{Context, Result};
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use reqwest::Client;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::Semaphore;
use url::Url;

/// 解析 M3U8 文件，返回 TS 片段的 URL 列表
async fn parse_m3u8(client: &Client, playlist_url: &str) -> Result<Vec<String>> {
    let response = client.get(playlist_url).send().await?;
    let content = response.text().await?;
    let base_url = Url::parse(playlist_url)?;

    let mut ts_urls = Vec::new();
    let re_ts = Regex::new(r"^([^#].*\.ts)")?; // 匹配 .ts 文件行

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if re_ts.is_match(line) {
            // 处理相对路径
            let full_url = base_url.join(line).context("Failed to join URL")?;
            ts_urls.push(full_url.to_string());
        }
    }

    Ok(ts_urls)
}

/// 下载单个 TS 片段，保存到临时目录，返回文件路径
async fn download_segment(
    client: &Client,
    url: &str,
    output_dir: &Path,
    index: usize,
    semaphore: Arc<Semaphore>,
) -> Result<PathBuf> {
    let _permit = semaphore.acquire().await?; // 控制并发数

    let filename = output_dir.join(format!("segment_{:05}.ts", index));
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download {}: {}", url, response.status());
    }

    let bytes = response.bytes().await?;
    tokio::fs::write(&filename, bytes).await?;

    Ok(filename)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <m3u8_url> <output_file> [concurrency]", args[0]);
        eprintln!("Example: {} https://example.com/playlist.m3u8 output.mp4 10", args[0]);
        std::process::exit(1);
    }

    let playlist_url = &args[1];
    let output_file = &args[2];
    let concurrency: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    println!("🔍 Parsing playlist...");
    let ts_urls = parse_m3u8(&client, playlist_url).await?;
    if ts_urls.is_empty() {
        anyhow::bail!("No TS segments found in playlist");
    }
    println!("✅ Found {} TS segments", ts_urls.len());

    // 创建临时目录存放片段
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path().to_path_buf();

    let semaphore = Arc::new(Semaphore::new(concurrency));
    let client = Arc::new(client);

    let pb = ProgressBar::new(ts_urls.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    println!("📥 Downloading {} segments (concurrency: {})...", ts_urls.len(), concurrency);

    // 并发下载
    let download_tasks = ts_urls
        .into_iter()
        .enumerate()
        .map(|(idx, url)| {
            let client = client.clone();
            let temp_path = temp_path.clone();
            let semaphore = semaphore.clone();
            let pb = pb.clone();
            tokio::spawn(async move {
                let result = download_segment(&client, &url, &temp_path, idx, semaphore).await;
                pb.inc(1);
                result
            })
        });

    let results = futures::future::join_all(download_tasks).await;

    // 收集下载成功的文件路径
    let mut segment_files = Vec::new();
    for result in results {
        match result {
            Ok(Ok(path)) => segment_files.push(path),
            Ok(Err(e)) => eprintln!("❌ Download error: {}", e),
            Err(e) => eprintln!("❌ Task join error: {}", e),
        }
    }

    pb.finish_with_message("Download complete");

    if segment_files.is_empty() {
        anyhow::bail!("No segments downloaded successfully");
    }

    println!("🔗 Merging {} segments into {}", segment_files.len(), output_file);
    // 合并文件（直接拼接）
    let mut output = File::create(output_file)?;
    for seg_path in segment_files.iter() {
        let mut seg_file = File::open(seg_path)?;
        io::copy(&mut seg_file, &mut output)?;
    }

    println!("✅ Done! Output saved to {}", output_file);
    // 临时目录会在 TempDir 析构时自动删除

    Ok(())
}