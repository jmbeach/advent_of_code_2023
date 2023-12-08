use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone)]
pub struct AlmanacToFromRange {
    pub to: i64,
    pub from: i64,
    pub range: i64,
}

impl AlmanacToFromRange {
    pub fn get_corresponding(&self, val: &i64) -> i64 {
        if self.is_in_from_range(val) {
            let distance = val - self.from;
            return self.to + distance
        }
        val.clone()
    }

    pub fn is_in_from_range(&self, val: &i64) -> bool {
        let end: &i64 = &(self.from + (self.range - 1));
        &self.from <= val && end >= val
    }
}

impl Eq for AlmanacToFromRange {}

impl PartialEq<Self> for AlmanacToFromRange {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
    }
}

impl PartialOrd<Self> for AlmanacToFromRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AlmanacToFromRange {
    fn cmp(&self, other: &Self) -> Ordering {
        other.from.cmp(&self.from)
    }
}

#[derive(Clone)]
pub struct AlmanacMap {
    pub source_title: String,
    pub destination_title: String,
    pub mapped: BinaryHeap<AlmanacToFromRange>,
}

pub struct SeedsAndMaps {
    pub seeds: Vec<i64>,
    pub maps: HashMap<String, AlmanacMap>,
}
