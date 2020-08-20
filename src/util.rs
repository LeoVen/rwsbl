use regex::Regex;
use scraper::Selector;

pub fn anchor_selector() -> Selector {
    Selector::parse("a").unwrap()
}

/// Currently Possible Numbers:
/// 123
/// 12300
/// 12.3
/// 123.0
/// .123
/// 0.123
/// 00.123
/// 0.0123
/// 00123
pub fn number_regex() -> Regex {
    Regex::new(r"([0-9]*\.?[0-9]+)").unwrap()
}

pub fn split(source: Vec<String>, chunks: usize) -> Vec<Vec<String>> {
    if source.len() == 0 {
        return vec![];
    }
    let chunk_size = source.len() / chunks + 1;
    let mut result = vec![vec![]];
    for item in source.into_iter() {
        let last = result.len() - 1;
        result[last].push(item);
        if result[last].len() == chunk_size {
            result.push(vec![]);
        }
    }
    return result;
}
