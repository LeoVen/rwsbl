mod multiset;
mod stats;
mod util;

use crate::multiset::Multiset;
use crate::stats::BenfordStats;
use crate::util::{anchor_selector, number_regex};
use regex;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::env;
use std::fmt::Debug;
use std::io::Read;
use url::{ParseError, Url};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!(
            "Oh no! Try again:\nUsage: {} [initial_url] [depth]",
            args.first().unwrap()
        );
        return;
    }

    let initial_url = &args[1];

    let mut depth = 4;
    if let Some(d) = &args.get(2) {
        depth = d.parse::<u32>().unwrap_or(4);
    }

    let client = reqwest::blocking::Client::new();
    let mut initial_response = client.get(initial_url).send().expect(&format!(
        "Could not get initial request from {}",
        initial_url
    ));
    let initial_body = initial_response.text().expect(&format!(
        "Could not get initial request text from {}",
        initial_url
    ));

    let document = Html::parse_document(&initial_body);
    let selector = anchor_selector();

    let mut initial_link_set = HashSet::new();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            if let Ok(_) = Url::parse(link) {
                initial_link_set.insert(link);
            }
        }
    }

    println!("Size: {}", initial_link_set.len());

    let mut set = Multiset::new();

    for cap in number_regex().captures_iter(&initial_body) {
        if let Some(m) = cap.get(1) {
            if let Some(c) = m.as_str().chars().nth(0) {
                if c != '0' {
                    set.insert(c.to_string());
                }
            }
        }
    }

    let mut result = BenfordStats::new(initial_url);

    for (key, val) in set.inner().iter() {
        result.set(key.parse::<usize>().unwrap() - 1, *val);
    }

    println!("{}", result);
}
