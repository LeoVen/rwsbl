#![allow(dead_code)]

mod cli;
mod multiset;
mod process;
mod stats;
mod util;

use crate::cli::{build_cli, get_args};
use crate::multiset::Multiset;
use crate::process::{get_data, get_html, get_links, thread_process};
use crate::stats::{BenfordStats, Stats};
use crate::util::{anchor_selector, number_regex, split};
use clap::{App, Arg};
use reqwest;
use scraper::Html;
use std::collections::HashSet;
use std::env;
use std::thread;
use url::Url;

fn main() {
    let cli = build_cli();
    let (url, depth, threads) = get_args(&cli);

    println!(
        "Running With:\n  Url: {}\n  Depth: {}\n  Threads: {}",
        url, depth, threads
    );

    let client = reqwest::blocking::Client::new();
    let initial_body = get_html(&client, &url).expect("Could not get initial HTML");
    let initial_url = Url::parse(url).expect("Could not parse initial URL");
    let initial_data = get_data(&initial_body, &initial_url);

    println!("{}", initial_data);

    // TODO Recursively get all links, saving the body. Then, process the body,
    // getting statistics on the numbers of the html body.
    return;

    // let mut set = Multiset::new();
    //
    // for cap in number_regex().captures_iter(&initial_body) {
    //     if let Some(m) = cap.get(1) {
    //         if let Some(c) = m.as_str().chars().nth(0) {
    //             if c != '0' {
    //                 set.insert(c.to_string());
    //             }
    //         }
    //     }
    // }
    //
    // let mut result = BenfordStats::new(initial_url.as_str());
    //
    // let total = set
    //     .inner()
    //     .iter()
    //     .fold(0.0, |acc, (_key, val)| *val as f64 + acc);
    //
    // for (key, val) in set.inner().iter() {
    //     result.set(
    //         key.parse::<usize>()
    //             .expect(&format!("Could not parse {} : {}", key, val))
    //             - 1,
    //         *val,
    //     );
    // }
    //
    // println!("{}", result);
    // for value in result.freq.0.iter() {
    //     println!("{} ", *value as f64 / total);
    // }
    //
    // let mut i = 0;
    // for link in &initial_link_set {
    //     println!("{}", link);
    //     if i > 10 {
    //         break;
    //     }
    //     i += 1;
    // }

    let total_links = initial_data.child_urls.len();
    let chunked_links = split(initial_data.child_urls.into_iter().collect(), 8);

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
