#![allow(dead_code)]

mod multiset;
mod process;
mod stats;
mod util;

use crate::multiset::Multiset;
use crate::process::thread_process;
use crate::stats::{BenfordStats, Stats};
use crate::util::{anchor_selector, number_regex, split};
use reqwest;
use scraper::Html;
use std::collections::HashSet;
use std::env;
use std::thread;
use url::Url;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Rust Web Scrapper for calculating Benford's Law on the web")
        .version("0.1.0")
        .author("Leonardo Vencovsky")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .takes_value(true)
            .help("The initial URL to start scrapping the numbers and links")
            .required(true)
            .multiple(false)
            .validator(|v| {
                if Url::parse(&v).is_ok() {
                    Ok(())
                } else {
                    Err(format!("Invalid argument url: {}", v))
                }
            }))
        .arg(Arg::with_name("depth")
            .short("d")
            .long("depth")
            .takes_value(true)
            .help("Each page have its links stored and then are requested recursively. This parameter defines the maximum depth of this recursion.")
            .required(false)
            .multiple(false)
            .default_value("1")
            .validator(|v| {
                if v.parse::<usize>().is_ok() {
                    Ok(())
                } else {
                    Err(format!("Invalid argument depth: {}", v))
                }
            }))
        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .help("How many threads to spawn. The initial links are distributed to each thread.")
            .required(false)
            .multiple(false)
            .default_value("8")
            .validator(|v| {
                if v.parse::<usize>().is_ok() {
                    Ok(())
                } else {
                    Err(format!("Invalid argument threads: {}", v))
                }
            }))
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let depth = matches.value_of("depth").unwrap_or("1").parse::<usize>().expect("Could not parse argument: depth");
    let threads = matches.value_of("threads").unwrap_or("8").parse::<usize>().expect("Could not parse argument: threads");

    println!("{} {} {}", url, depth, threads);
    return;

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
        depth = d.parse::<usize>().unwrap_or(4);
    }

    let client = reqwest::blocking::Client::new();
    let initial_response = client.get(initial_url).send().expect(&format!(
        "Could not get initial request from {}",
        initial_url
    ));
    let initial_body = initial_response.text().expect(&format!(
        "Could not get initial request text from {}",
        initial_url
    ));

    let document = Html::parse_document(&initial_body);
    let selector = anchor_selector();

    let mut initial_link_set: HashSet<String> = HashSet::new();

    let initial_url = Url::parse(initial_url).expect("Could not parse initial URL");

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            if link.contains('#') {
                // We don't want links that take us to the same page
                continue;
            }
            let mut full_path = link.to_string();
            if !link.starts_with("http") {
                full_path = initial_url.as_str().replace(initial_url.path(), "") + &full_path;
            }
            if let Ok(url) = Url::parse(&full_path) {
                initial_link_set.insert(url.as_str().to_string());
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

    let mut result = BenfordStats::new(initial_url.as_str());

    let total = set
        .inner()
        .iter()
        .fold(0.0, |acc, (_key, val)| *val as f64 + acc);

    for (key, val) in set.inner().iter() {
        result.set(
            key.parse::<usize>()
                .expect(&format!("Could not parse {} : {}", key, val))
                - 1,
            *val,
        );
    }

    println!("{}", result);
    for value in result.freq.0.iter() {
        println!("{} ", *value as f64 / total);
    }

    let mut i = 0;
    for link in &initial_link_set {
        println!("{}", link);
        if i > 10 {
            break;
        }
        i += 1;
    }

    let total_links = initial_link_set.len();
    let chunked_links = split(initial_link_set.into_iter().collect(), 8);

    let mut children = Vec::new();

    for chunk in chunked_links.into_iter() {
        children.push(thread::spawn(move || thread_process(chunk, depth)));
    }

    let result = children
        .into_iter()
        .map(|c| c.join().unwrap())
        .collect::<Vec<Stats>>();

    println!("\nTotal Initial Links {}", total_links);
    println!(
        "Total  Result {:>3}",
        result
            .iter()
            .fold(0, |acc, stats| acc + stats.url_stats.len())
    );
    println!(
        "Total Success {:>3}",
        result.iter().fold(0, |acc, stats| acc + stats.success)
    );
    println!(
        "Total   Fails {:>3}",
        result.iter().fold(0, |acc, stats| acc + stats.fail)
    );
}
