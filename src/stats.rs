use crate::multiset::Multiset;
use std::fmt;
use std::fmt::Formatter;

struct ArrayMap([usize; 9]);

/// Stores relevant data about a certain URL.
pub struct BenfordStats<'a> {
    /// The url where the stats were taken.
    url: &'a str,
    /// Initial number frequency. 0 is not used.
    freq: ArrayMap,
    /// Maps the size of numbers to their frequency.
    size_freq: Multiset<u64>,
}

impl<'a> BenfordStats<'a> {
    pub fn new(url: &'a str) -> Self {
        Self {
            url,
            freq: ArrayMap([0; 9]),
            size_freq: Multiset::new(),
        }
    }

    pub fn set(&mut self, key: usize, value: usize) {
        self.freq.0[key] = value;
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
