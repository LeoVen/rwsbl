use crate::multiset::Multiset;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

/// Stores relevant data about all URLs
pub struct Stats<'a> {
    pub url_stats: Vec<BenfordStats<'a>>,
    pub success: usize,
    pub fail: usize,
}

pub struct ArrayMap(pub [usize; 9]);

/// Stores relevant data about a certain URL.
pub struct BenfordStats<'a> {
    /// The URL where the stats were taken.
    pub url: &'a str,
    /// The URL's child URLs
    pub child_urls: HashSet<String>,
    /// Initial number frequency. 0 is not used.
    pub freq: ArrayMap,
    /// Maps the size of numbers to their frequency.
    pub size_freq: Multiset<u64>,
}

impl<'a> BenfordStats<'a> {
    pub fn set(&mut self, key: usize, value: usize) {
        self.freq.0[key] = value;
    }
}

impl<'a> Default for BenfordStats<'a> {
    fn default() -> Self {
        Self {
            url: "",
            child_urls: HashSet::default(),
            freq: ArrayMap([0; 9]),
            size_freq: Multiset::default(),
        }
    }
}

impl fmt::Display for ArrayMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}, {}, {}, {}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8]
        )
    }
}

impl<'a> fmt::Display for BenfordStats<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\", {}", self.url, self.freq)
    }
}
