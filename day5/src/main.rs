use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct AlmanacMap {
    pub source_title: String,
    pub destination_title: String,
    mapped: HashMap<i32, i32>,
}

impl AlmanacMap {
    pub fn add_range(mut self, source_start: i32, dest_start: i32, range_size: i32) {
        for i in 0..range_size {
            self.mapped.insert(source_start + i, dest_start + i);
        }
    }

    pub fn get(&self, key: &i32) -> &i32 {
        self.mapped.get(key).unwrap_or(key)
    }
}

fn parse_input(input: &str) -> HashMap<String, AlmanacMap> {
    let result = HashMap::new();
    let line_i = 0;
    let lines: Vec<&str> = input.lines().collect();
    let mut current_almanac: AlmanacMap;
    while line_i < lines.count() {
        let line = lines[line_i];
        match line {
            line if line.starts_with("") => {
                let parts: Vec<&str> = line.replace(" map:", "").split("-to-").collect();
                current_almanac = AlmanacMap {
                    source_title: String::from(parts[0]),
                    destination_title: String::from(parts[1]),
                    mapped: HashMap::new(),
                }
            }
            _ => {}
        }
    }
    result
}

