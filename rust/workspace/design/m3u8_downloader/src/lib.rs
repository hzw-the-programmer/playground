use anyhow::{Context, Result};
use regex::Regex;
use reqwest::Client;
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
