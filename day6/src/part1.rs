fn main() {
    println!("Hello, world!");
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let lines: Vec<&str> = input.lines().collect();
    let result: Vec<Vec<i64>> = lines.into_iter()
        .map(|line| line.split_whitespace()
            .filter_map(|word| word.parse::<i64>().ok())
            .collect::<Vec<i64>>())
        .collect();
    (result[0].clone(), result[1].clone())
}

pub fn part1(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let max_distance_press = time / 2;
        // let max_distance = max_distance_press * (time - max_distance_press);
        let actual_time_pressed = (distance - time) / 2;
        // it's the distance between the number pressed and the best number pressed * 2
        // + 1 if it's an odd time.
        return (max_distance_press - actual_time_pressed) * 2;
    }
}

#[cfg(test)]
mod tests6 {
    use crate::part1::parse_input;

    #[test]
    fn test() {
        let parsed = parse_input("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(parsed, (vec!(7, 15, 30), vec!(9, 40, 200)))
    }
}