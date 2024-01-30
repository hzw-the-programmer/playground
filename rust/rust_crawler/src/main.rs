use rayon::prelude::*;
use reqwest::Url;
use select::document::Document;
use select::predicate::Name;
use select::predicate::Predicate;
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::time::Instant;

// https://rolisz.ro/2020/03/01/web-crawler-in-rust/

fn main() {
    let now = Instant::now();

    let client = reqwest::blocking::Client::new();
    let origin_url = "https://rolisz.ro/";

    let body = fetch_url(&client, origin_url).unwrap();
    write_file("", &body);

    let mut visited = HashSet::new();
    visited.insert(origin_url.to_string());
    let found_urls = get_links_from_html(&body);
    let mut new_urls = found_urls
        .difference(&visited)
        .map(|x| x.to_string())
        .collect::<HashSet<String>>();

    while !new_urls.is_empty() {
        let (found_urls, errors): (Vec<Result<HashSet<String>>>, Vec<_>) = new_urls
            .par_iter()
            .map(|url| -> Result<HashSet<String>> {
                let body = fetch_url(&client, url)?;
                write_file(&url[origin_url.len() - 1..], &body)?;

                let links = get_links_from_html(&body);
                println!("Visited: {} found {} links", url, links.len());

                Ok(links)
            })
            .partition(Result::is_ok);

        visited.extend(new_urls);

        new_urls = found_urls
            .into_par_iter()
            .map(Result::unwrap)
            .reduce(HashSet::new, |mut acc, x| {
                acc.extend(x);
                acc
            })
            .difference(&visited)
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        println!("New urls: {}", new_urls.len());
        println!(
            "Errors: {:#?}",
            errors
                .into_iter()
                .map(Result::unwrap_err)
                .collect::<Vec<Error>>()
        );
    }

    println!("URLs: {:#?}", found_urls);
    println!("{}", now.elapsed().as_secs());
}

fn get_links_from_html(html: &str) -> HashSet<String> {
    Document::from(html)
        .find(Name("a").or(Name("link")))
        .filter_map(|n| n.attr("href"))
        .filter(has_extension)
        .filter_map(normalize_url)
        .collect::<HashSet<String>>()
}

fn normalize_url(url: &str) -> Option<String> {
    let new_url = Url::parse(url);
    match new_url {
        Ok(new_url) => {
            if let Some("ghost.rolisz.ro") = new_url.host_str() {
                Some(url.to_string())
            } else {
                None
            }
        }
        Err(_e) => {
            // Relative urls are not parsed by Reqwest
            if url.starts_with('/') {
                Some(format!("https://rolisz.ro{}", url))
            } else {
                None
            }
        }
    }
}

fn fetch_url(client: &reqwest::blocking::Client, url: &str) -> Result<String> {
    let mut res = client.get(url).send().map_err(|e| (url, e))?;
    println!("Status for {}: {}", url, res.status());

    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|e| (url, e))?;
    Ok(body)
}

fn write_file(path: &str, content: &str) -> Result<()> {
    let dir = format!("static{}", path);
    fs::create_dir_all(&dir).map_err(|e| (&dir, e))?;
    let index = dir + "/index.html";
    fs::write(&index, content).map_err(|e| (&index, e))?;
    Ok(())
}

fn has_extension(url: &&str) -> bool {
    Path::new(url).extension().is_none()
}

#[derive(Debug)]
enum Error {
    Write { url: String, e: std::io::Error },
    Fetch { url: String, e: reqwest::Error },
}

type Result<T> = std::result::Result<T, Error>;

impl<S: AsRef<str>> From<(S, std::io::Error)> for Error {
    fn from((url, e): (S, std::io::Error)) -> Self {
        Error::Write {
            url: url.as_ref().to_string(),
            e,
        }
    }
}

impl<S: AsRef<str>> From<(S, reqwest::Error)> for Error {
    fn from((url, e): (S, reqwest::Error)) -> Self {
        Error::Fetch {
            url: url.as_ref().to_string(),
            e,
        }
    }
}
