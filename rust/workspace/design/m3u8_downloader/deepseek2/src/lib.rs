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
pub async fn parse_m3u8(client: &Client, playlist_url: &str) -> Result<Vec<String>> {
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
pub async fn download_segment(
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

/// 主下载函数，返回临时目录和下载成功的片段文件路径列表
pub async fn download_all_segments(
    client: &Client,
    ts_urls: Vec<String>,
    concurrency: usize,
) -> Result<(TempDir, Vec<PathBuf>)> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path().to_path_buf();

    let semaphore = Arc::new(Semaphore::new(concurrency));
    let client = Arc::new(client.clone());

    let pb = ProgressBar::new(ts_urls.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

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

    Ok((temp_dir, segment_files))
}

/// 合并片段文件到输出路径
pub fn merge_segments(segment_files: &[PathBuf], output_path: &Path) -> Result<()> {
    let mut output = File::create(output_path)?;
    for seg_path in segment_files {
        let mut seg_file = File::open(seg_path)?;
        io::copy(&mut seg_file, &mut output)?;
    }
    Ok(())
}