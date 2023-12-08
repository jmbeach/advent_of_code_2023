use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use structs::{AlmanacMap, AlmanacToFromRange, SeedsAndMaps};

mod structs;

fn main() {
    println!("Hello, world!");
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
    let mut locations: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for seed in seeds_and_maps.seeds {
        let mut from_value = seed.clone();
        let mut from_key: String = String::from("seed");
        while from_key.as_str() != "location" {
            let mut current_map: AlmanacMap = seeds_and_maps.maps.get(from_key.as_str()).unwrap().clone();
            let mut next_range = current_map.mapped.pop();

            // short circuit when we get any potential match
            while next_range.is_some_and(|range| range.from + (range.range - 1) < from_value) {
                next_range = current_map.mapped.pop();
            }

            match next_range {
                // the seed maps directly to the "to". from_value stays the same.
                None => {},
                Some(range) => {
                    from_value = range.get_corresponding(&from_value);
                }
            }

            from_key = current_map.destination_title.clone();
        }
        locations.push(Reverse(from_value))
    }
    locations.pop().unwrap().0
}

#[cfg(test)]
pub mod tests5 {
    use crate::{find_smallest, parse_input};

    #[test]
    fn basic_test() {
        let mut seeds_and_maps = parse_input("seeds: 79 14 55 13

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
        assert_eq!(seeds_and_maps.seeds, vec!(79, 14, 55, 13));
        let mut almanac_map = seeds_and_maps.maps.get("seed").unwrap().clone();
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
        let smallest_location = find_smallest(seeds_and_maps);
        assert_eq!(smallest_location, 35);
    }
}
