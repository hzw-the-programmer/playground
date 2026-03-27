use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use reqwest::Client;
use tempfile::TempDir;
use tokio::sync::Semaphore;
use url::Url;

pub async fn parse_m3u8(client: &Client, playlist_url: &str) -> Result<Vec<String>> {
    let response = client.get(playlist_url).send().await?;
    let content = response.text().await?;
    let base_url = Url::parse(playlist_url)?;

    let mut ts_urls = Vec::new();
    let re_ts = Regex::new(r"^([^#].*\.ts)")?;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if re_ts.is_match(line) {
            let full_url = base_url.join(line).context("Failed to join URL")?;
            ts_urls.push(full_url.to_string());
        }
    }

    Ok(ts_urls)
}

pub async fn download_segment(
    client: &Client,
    url: &str,
    output_dir: &Path,
    index: usize,
    semaphore: Arc<Semaphore>,
) -> Result<PathBuf> {
    let _permit = semaphore.acquire().await?;

    let resp = client.get(url).send().await?;
    if !resp.status().is_success() {
        anyhow::bail!("Failed to download {}: {}", url, resp.status());
    }

    let bytes = resp.bytes().await?;
    let filename = output_dir.join(format!("segment_{:05}.ts", index));
    tokio::fs::write(&filename, bytes).await?;

    Ok(filename)
}

pub async fn merge_segments(segment_files: &[PathBuf], output_path: &Path) -> Result<()> {
    let mut output = tokio::fs::File::create(output_path).await?;
    for seg_path in segment_files {
        let mut seg_file = tokio::fs::File::open(seg_path).await?;
        tokio::io::copy(&mut seg_file, &mut output).await?;
    }
    Ok(())
}

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
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let download_tasks = ts_urls.into_iter().enumerate().map(|(idx, url)| {
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

    let mut segment_files = vec![];
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
