mod cli;
mod multiset;
mod process;
mod stats;
mod util;

use crate::cli::{build_cli, get_args};
use crate::process::{get_data, get_html, thread_process};
use crate::stats::{ArrayMap, Stats};
use crate::util::{num_to_letter_ascii, split};
use reqwest;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::iter::FromIterator;
use std::thread;
use url::Url;

// TODO get the HTML from each URL, join it all into a HashMap and then calculate the numbers to
// avoid accessing repeating URLs.
// TODO join threads after each depth so that threads that have exhausted links can go back to work
// and could also avoid mid-process access to repeating URLs.
// TODO add out.csv as parameter

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

    let mut freq_start = ArrayMap([0; 9]);
    freq_start.merge(&initial_data.freq_start);
    let mut freq_end = ArrayMap([0; 9]);
    freq_end.merge(&initial_data.freq_end);
    let mut pass = 0;
    let mut fail = 0;

    let total_links = initial_data.child_urls.len();
    let chunked_links = split(initial_data.child_urls.into_iter().collect(), 8);

    println!("Initial links: {}", total_links);

    let mut children = Vec::new();

    for (i, chunk) in chunked_links.into_iter().enumerate() {
        children.push(thread::spawn(move || {
            thread_process(
                HashSet::from_iter(chunk.into_iter()),
                depth,
                num_to_letter_ascii(i),
            )
        }));
    }

    let result = children
        .into_iter()
        .map(|c| c.join().unwrap())
        .collect::<Vec<Stats>>();

    for r in result.iter() {
        for s in r.url_stats.iter() {
            freq_start.merge(&s.freq_start);
            freq_end.merge(&s.freq_end);
        }
        pass += r.success;
        fail += r.fail;
    }

    println!("\nTotal Initial Links {}", total_links);
    println!("Total  Result {:>3}", pass + fail);
    println!("Total Success {:>3}", pass);
    println!("Total   Fails {:>3}", fail);

    println!("{}\n{}", freq_start, freq_end);

    if let Ok(mut file) = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("/out.csv")
    {
        'outer: for stats in result.iter() {
            for url_stats in stats.url_stats.iter() {
                if file.write(format!("{}", url_stats).as_bytes()).is_err() {
                    println!("Failed to write everything to file output");
                    break 'outer;
                }
            }
        }
    } else {
        eprintln!("Failed to open file for csv output");
    }
}
