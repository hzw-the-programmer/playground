use reqwest::Url;
use select::document::Document;
use select::predicate::Name;
use select::predicate::Predicate;
use std::collections::HashSet;
use std::io::Read;

fn main() {
    let client = reqwest::blocking::Client::new();
    let origin_url = "https://rolisz.ro/";
    let mut res = client.get(origin_url).send().unwrap();
    println!("Status for {}: {}", origin_url, res.status());

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("HTML: {}", &body[0..40]);

    let found_urls = get_links_from_html(&body);
    println!("URLs: {:#?}", found_urls)
}

fn get_links_from_html(html: &str) -> HashSet<String> {
    Document::from(html)
        .find(Name("a").or(Name("link")))
        .filter_map(|n| n.attr("href"))
        .filter_map(normalize_url)
        .collect::<HashSet<String>>()
}

fn normalize_url(url: &str) -> Option<String> {
    let new_url = Url::parse(url);
    match new_url {
        Ok(new_url) => {
            if new_url.has_host() && new_url.host_str().unwrap() == "ghost.rolisz.ro" {
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
