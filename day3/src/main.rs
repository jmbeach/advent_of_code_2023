use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

fn main() {
    let mut schematic = Schematic::new(fs::read_to_string("./input.txt").unwrap().as_str());
    schematic.get_part_numbers();
    let sum = schematic.sum;
    println!("Result: {sum}")
}

struct Schematic {
    pub grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub found_coordinates: HashSet<(usize, usize)>,
    pub part_numbers: Vec<i32>,
    pub sum: i32
}

impl Schematic {
    pub fn new(text_grid: &str) -> Self {
        Self {
            grid: text_grid.lines().map(|line| line.chars().collect()).collect(),
            width: text_grid.lines().next().unwrap().len(),
            height: text_grid.lines().count(),
            found_coordinates: HashSet::new(),
            part_numbers: vec!(),
            sum: 0
        }
    }

    pub fn get_part_numbers(&mut self) -> Vec<i32> {
        for row in 0..self.height {
            for col in 0..self.width {
                let current = self.grid[row][col];
                match current {
                    '.' | '0'..='9' => {}, // do nothing
                    _ => self.check_coordinate(col, row)
                }
            }
        }
        self.part_numbers.clone()
    }

    fn check_coordinate(&mut self, x: usize, y: usize) {
        for row in max(y - 1, 0)..=min(y + 1, self.height - 1) {
            for col in max(x - 1, 0)..=min(x + 1, self.width - 1) {
                // don't check the passed in coordinate
                if x == col && y == row {
                    continue
                }
                let current = self.grid[row][col];
                match current {
                    '0'..='9' => self.sum += self.process_part_number(col, row),
                    _ => {} // do nothing
                }
            }
        }
    }

    fn process_part_number(&mut self, x: usize, y: usize) -> i32 {
        if self.found_coordinates.contains(&(x, y)) {
            return 0
        }

        // walk left until at start of number
        let mut start_x = x;
        while start_x > 0 {
            match self.grid[y][start_x - 1] {
                '0'..='9' => start_x -= 1,
                _ => break
            }
        }

        let mut number = String::from("");
        for col in start_x..self.width {
            let current = self.grid[y][col];
            match current {
                '0'..='9' => {
                    number.push(current);
                    self.found_coordinates.insert((col, y));
                },
                _ => break
            }
        }
        let result = number.parse::<i32>().unwrap();
        self.part_numbers.push(result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Schematic;

    #[test]
    fn basic_test() {
        let example_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let mut schematic = Schematic::new(example_input);
        schematic.get_part_numbers();
        assert_eq!(schematic.sum, 4361)
    }
}