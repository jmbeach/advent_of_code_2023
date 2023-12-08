use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    println!("Hello, world!");
}

struct AlmanacToFromRange {
    pub to: i32,
    pub from: i32,
    pub range: i32,
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

struct AlmanacMap {
    pub source_title: String,
    pub destination_title: String,
    mapped: BinaryHeap<AlmanacToFromRange>,
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

impl Clone for AlmanacMap {
    fn clone(&self) -> Self {
        AlmanacMap {
            source_title: self.source_title.clone(),
            destination_title: self.destination_title.clone(),
            mapped: self.mapped.clone()
        }
    }
}

impl Copy for AlmanacMap {}

struct SeedsAndMaps {
    seeds: Vec<i32>,
    maps: HashMap<String, AlmanacMap>,
}

fn parse_input(input: &str) -> SeedsAndMaps {
    let mut maps: HashMap<String, AlmanacMap> = HashMap::new();

    let lines: Vec<&str> = input.lines().collect();
    // first line is seeds
    let seeds: Vec<i32> = lines[0].replace("seeds: ", "").trim()
        .split(" ").map(|num| num.parse::<i32>().unwrap()).collect();
    let mut line_i = 1;
    let mut current_from: String = String::from("");
    while line_i < lines.len() {
        let line = lines[line_i];
        match line {
            // example: seed-to-soil map:
            line if line.ends_with("map:") => {
                let parts_raw = line.replace(" map:", "");
                let parts: Vec<&str> = parts_raw.split("-to-").collect();
                let from = String::from(parts[0].clone());
                current_from = from.clone();
                maps.insert(from.clone(), AlmanacMap {
                    source_title: from,
                    destination_title: String::from(parts[1]),
                    mapped: BinaryHeap::new(),
                });
            }
            "" => {}
            // ex: 50 98 2
            _ => {
                let nums: Vec<i32> = line.split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap()).collect();
                let current = maps.get_mut(current_from.as_str()).unwrap();
                current.mapped.push(AlmanacToFromRange {
                    to: nums[0],
                    from: nums[1],
                    range: nums[2],
                })
            }
        }
        line_i += 1;
    }
    SeedsAndMaps {
        seeds,
        maps,
    }
}

fn find_smallest(seeds_and_maps: SeedsAndMaps) -> i32 {
    let mut locations: BinaryHeap<i32> = BinaryHeap::new();
    let mut seed_map: AlmanacMap = seeds_and_maps.maps.get("seed").unwrap().clone();
    for seed in seeds_and_maps.seeds {
        let mut next_range = seed_map.mapped.pop();
        // short circuit when we get any potential match
        while next_range.is_some_and(|range| !range.from + range.range < seed) {
            next_range = seed_map.mapped.pop();
        }
        match next_range {
            None => {}, // the seed maps directly to the "to"
            Some(range) => {} // check the range.
        }
    }
    locations.pop().unwrap()
}

#[cfg(test)]
pub mod tests5 {
    use crate::parse_input;

    #[test]
    fn basic_test() {
        let mut seedsAndMaps = parse_input("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        assert_eq!(seedsAndMaps.seeds, vec!(79, 14, 55, 13));
        let almanac_map = seedsAndMaps.maps.get_mut("seed").unwrap();
        assert_eq!(almanac_map.destination_title, "soil");
        assert_eq!(almanac_map.source_title, "seed");
        let first_range = almanac_map.mapped.pop().unwrap();
        assert_eq!(first_range.from, 50);
        assert_eq!(first_range.to, 52);
        assert_eq!(first_range.range, 48);
        let next_range = almanac_map.mapped.pop().unwrap();
        assert_eq!(next_range.from, 98);
        assert_eq!(next_range.to, 50);
        assert_eq!(next_range.range, 2);
    }
}