use anyhow::Result;
use reqwest::Client;

use m3u8_downloader::{download_all_segments, merge_segments, parse_m3u8};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <m3u8_url> <output_file> [concurrency]", args[0]);
        eprintln!(
            "Example: {} https://example.com/playlist.m3u8 output.mp4 10",
            args[0]
        );
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

    println!("📥 Downloading segments...");
    let (_temp_dir, segment_files) = download_all_segments(&client, ts_urls, concurrency).await?;

    println!("🔗 Merging segments...");
    merge_segments(&segment_files, output_file.as_ref()).await?;

    println!("✅ Done! Output saved to {}", output_file);
    Ok(())
}
