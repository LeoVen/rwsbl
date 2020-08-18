use regex::Regex;
use scraper::Selector;

pub fn anchor_selector() -> Selector {
    Selector::parse("a").unwrap()
}

pub fn number_regex() -> Regex {
    Regex::new(r"(\d+)").unwrap()
}
