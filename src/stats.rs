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

pub enum FreqType {
    Start,
    End,
}

pub struct ArrayMap(pub [usize; 9]);

/// Stores relevant data about a certain URL.
pub struct BenfordStats<'a> {
    /// The URL where the stats were taken.
    pub url: &'a str,
    /// The URL's child URLs
    pub child_urls: HashSet<String>,
    /// First number frequency. 0 is not used.
    pub freq_start: ArrayMap,
    /// Last number frequency. 0 is not used.
    pub freq_end: ArrayMap,
    /// Maps the size of numbers to their frequency.
    pub size_freq: Multiset<u64>,
}

impl<'a> BenfordStats<'a> {
    pub fn add(&mut self, key: usize, freq_type: FreqType) {
        if key == 0 {
            // Error?
            return;
        }
        match freq_type {
            FreqType::Start => self.freq_start.0[key - 1] += 1,
            FreqType::End => self.freq_end.0[key - 1] += 1,
        }
    }
}

impl<'a> Default for BenfordStats<'a> {
    fn default() -> Self {
        Self {
            url: "",
            child_urls: HashSet::default(),
            freq_start: ArrayMap([0; 9]),
            freq_end: ArrayMap([0; 9]),
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
        write!(
            f,
            "\"{}\", {}, {}",
            self.url, self.freq_start, self.freq_end
        )
    }
}
