use crate::stats::{BenfordStats, Stats};
use std::io::{stdout, Write};

pub fn thread_process<'a>(links: Vec<String>, _depth: usize) -> Stats<'a> {
    println!("Spawned Thread from Process {}", std::process::id());
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut success = 0;
    let mut fail = 0;
    let mut result = vec![];

    let client = reqwest::blocking::Client::new();

    for link in links.iter() {
        let _ = print!("+");
        let _ = stdout().flush();
        match client.get(link).send() {
            Ok(_) => success += 1,
            Err(_) => fail += 1,
        }
    }

    Stats {
        success,
        fail,
        url_stats: result,
    }
}
