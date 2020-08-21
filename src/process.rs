use crate::stats::{BenfordStats, FreqType, Stats};
use crate::util::{anchor_selector, number_regex};
use reqwest::blocking::Client;
use scraper::Html;
use std::collections::HashSet;
use std::io::{stdout, Write};
use url::Url;

pub fn thread_process<'a>(links: HashSet<String>, depth: usize, ident: &'static str) -> Stats {
    println!(
        "Spawned Thread from Process {}, received {}",
        std::process::id(),
        links.len()
    );
    std::thread::sleep(std::time::Duration::from_secs(1));
    build_stats(
        &reqwest::blocking::Client::new(),
        Stats::new(Some(links.len())),
        &links,
        depth,
        ident,
    )
}

pub fn build_stats<'a>(
    client: &Client,
    stats: Stats,
    links: &HashSet<String>,
    depth: usize,
    ident: &'static str,
) -> Stats {
    if depth == 0 {
        return stats;
    }
    let mut result = Stats::new(Some(links.len()));
    for url in links.iter() {
        if let Ok(html) = get_html(client, url) {
            if let Ok(url) = Url::parse(url) {
                let data = get_data(&html, &url);
                result = build_stats(client, result, &data.child_urls, depth - 1, ident);
                result.add(data);
                let _ = print!("[+{}]", ident);
            } else {
                let _ = print!("[-{}]", ident);
                result.fail();
            }
        } else {
            let _ = print!("[-{}]", ident);
            result.fail();
        }
        let _ = stdout().flush();
    }
    stats.merge(result)
}

/// Tries to get the html content of a URL, reusing a provided client
pub fn get_html(client: &Client, url: &str) -> Result<String, ()> {
    match client.get(url).send() {
        Ok(response) => response.text().or(Err(())),
        Err(_) => Err(()),
    }
}

/// Retrieves all links from an html page using regex.
/// # Parameters
/// - html: The HTML to be scanned
/// - url: The link that was used to get HTML
pub fn get_links(html: &str, url: &Url) -> HashSet<String> {
    let document = Html::parse_document(html);
    let mut result = HashSet::new();
    for element in document.select(&anchor_selector()) {
        if let Some(link) = element.value().attr("href") {
            if link.contains('#') {
                // We don't want links that take us to the same page
                continue;
            }
            let mut full_path = link.to_string();
            if !link.starts_with("http") {
                // We might be able to build a relative path with the original url
                full_path = url.as_str().replace(url.path(), "") + &full_path;
            }
            if let Ok(url) = Url::parse(&full_path) {
                result.insert(url.as_str().to_string());
            }
        }
    }
    result
}

/// Gets all useful data from the HTML acquired by the URL
pub fn get_data<'a>(html: &'a str, url: &'a Url) -> BenfordStats {
    let mut result = BenfordStats::default();
    result.url = url.as_str().to_string();
    result.child_urls = get_links(html, url);
    for cap in number_regex().captures_iter(html) {
        if let Some(m) = cap.get(1) {
            let number = treat_number(m.as_str().to_string());
            result.size_freq.insert(number.len() as u64);
            if number.len() > 0 {
                let first = number.chars().nth(0);
                let last = number.chars().nth(number.len() - 1);
                if first.is_some() && last.is_some() {
                    let first = first.unwrap().to_digit(10);
                    let last = last.unwrap().to_digit(10);
                    if first.is_some() && last.is_some() {
                        result.add(first.unwrap() as usize, FreqType::Start);
                        result.add(last.unwrap() as usize, FreqType::End);
                    }
                }
            }
        }
    }
    result
}

/// Treat edge cases for numbers with trailing 0s and punctuation
/// > See number_regex()
/// 123    -> 123
/// 12300  -> 123
/// 12.3   -> 123
/// 123.0  -> 123
/// .123   -> 123
/// 0.123  -> 123
/// 00.123 -> 123
/// 0.0123 -> 123
/// 00123  -> 123
pub fn treat_number(mut number: String) -> String {
    number = number.replace(".", "");
    while number.len() > 0 && number.chars().nth(number.len() - 1) == Some('0') {
        let _ = number.pop();
    }
    let mut start = 0;
    while number.len() > 0 && number.chars().nth(start) == Some('0') {
        start += 1;
    }
    if number.len() > 0 {
        number.drain(..start);
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn treat_number_test() {
        assert_eq!("123", treat_number("123".to_string()));
        assert_eq!("123", treat_number("12300".to_string()));
        assert_eq!("123", treat_number("12.3".to_string()));
        assert_eq!("123", treat_number("123.0".to_string()));
        assert_eq!("123", treat_number(".123".to_string()));
        assert_eq!("123", treat_number("0.123".to_string()));
        assert_eq!("123", treat_number("00.123".to_string()));
        assert_eq!("123", treat_number("0.0123".to_string()));
        assert_eq!("123", treat_number("00123".to_string()));
    }
}
