use crate::multiset::Multiset;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

/// Stores relevant data about all URLs
pub struct Stats {
    pub url_stats: Vec<BenfordStats>,
    pub success: u128,
    pub fail: u128,
}

impl Stats {
    pub fn new(capacity: Option<usize>) -> Self {
        Self {
            url_stats: Vec::with_capacity(capacity.unwrap_or(32)),
            success: 0,
            fail: 0,
        }
    }

    pub fn add(&mut self, stats: BenfordStats) {
        self.url_stats.push(stats);
        self.pass();
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.url_stats.extend(other.url_stats.into_iter());
        self.success += self.success;
        self.fail += self.fail;
        self
    }

    pub fn fail(&mut self) {
        self.fail += 1;
    }

    pub fn pass(&mut self) {
        self.success += 1;
    }
}

pub enum FreqType {
    Start,
    End,
}

pub struct ArrayMap(pub [u128; 9]);

impl ArrayMap {
    pub fn merge(&mut self, other: &Self) {
        for i in 0..self.0.len() {
            self.0[i] = other.0[i];
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

/// Stores relevant data about a certain URL.
pub struct BenfordStats {
    /// The URL where the stats were taken.
    pub url: String,
    /// The URL's child URLs
    pub child_urls: HashSet<String>,
    /// First number frequency. 0 is not used.
    pub freq_start: ArrayMap,
    /// Last number frequency. 0 is not used.
    pub freq_end: ArrayMap,
    /// Maps the size of numbers to their frequency.
    pub size_freq: Multiset<u64>,
}

impl BenfordStats {
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

impl Default for BenfordStats {
    fn default() -> Self {
        Self {
            url: String::new(),
            child_urls: HashSet::default(),
            freq_start: ArrayMap([0; 9]),
            freq_end: ArrayMap([0; 9]),
            size_freq: Multiset::default(),
        }
    }
}

impl<'a> fmt::Display for BenfordStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\"{}\", {}, {}",
            self.url, self.freq_start, self.freq_end
        )
    }
}
