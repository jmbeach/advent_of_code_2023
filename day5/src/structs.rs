use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone)]
pub struct AlmanacToFromRange {
    pub to: i32,
    pub from: i32,
    pub range: i32,
}

impl AlmanacToFromRange {
    pub fn get_corresponding(&self, val: &i32) -> i32 {
        if self.is_in_from_range(val) {
            let distance = val - self.from;
            return self.to + distance
        }
        val.clone()
    }

    pub fn is_in_from_range(&self, val: &i32) -> bool {
        let end: &i32 = &(self.from + (self.range - 1));
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

impl AlmanacMap {
    pub fn add_range(mut self, source_start: i32, dest_start: i32, range_size: i32) {
        self.mapped.push(AlmanacToFromRange {
            from: source_start,
            to: dest_start,
            range: range_size,
        });
    }
}

pub struct SeedsAndMaps {
    pub seeds: Vec<i32>,
    pub maps: HashMap<String, AlmanacMap>,
}
