mod pt2;

use std::fs;

fn main() {
    let raw_text = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = raw_text.lines().map(|x| x).collect();
    let result = sum_calibrations(clean_calibrations(&lines));
    println!("Result: {result}")
}

fn clean_calibrations(input: &Vec<&str>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for dirty_calibration in input {
        result.push(clean_calibration(dirty_calibration))
    }
    return result;
}

fn clean_calibration_naive(dirty_text: &str) -> (i32, i32) {
    let mut first_int: Option<i32> = None;
    let mut second_int: Option<i32> = None;
    for char in dirty_text.chars() {
        match char {
            '0'..='9' => {
                let as_int = char.to_string().parse::<i32>().expect("Could not convert char to int");
                if first_int.is_none() {
                    first_int = Some(as_int)
                } else {
                    second_int = Some(as_int)
                }
            }
            _ => {} // do nothing
        }
    }
    if first_int.is_none() {
        panic!("There are no numbers in the dirty text: [{dirty_text}]")
    }
    match second_int {
        None => (first_int.unwrap(), first_int.unwrap()),
        Some(calibration) => (first_int.unwrap(), calibration)
    }
}

fn clean_calibration(dirty_text: &str) -> (i32, i32) {
    let mut left = 0;
    let mut right = dirty_text.len() - 1;
    let mut first_int: Option<i32> = None;
    let mut second_int: Option<i32> = None;
    let chars_vec: Vec<char> = dirty_text.chars().collect();
    while left <= right && (first_int.is_none() || second_int.is_none()) {
        let left_char: char = chars_vec[left];
        let right_char: char = chars_vec[right];
        if left_char.is_numeric() {
            first_int = Some(left_char.to_string().parse::<i32>()
                .expect("Could not convert left char to int"));
        }
        if right_char.is_numeric() {
            second_int = Some(right_char.to_string().parse::<i32>()
                .expect("Could not convert right char to int"));
        }
        if left_char.is_numeric() && right_char.is_numeric() {
            break
        } else if left_char.is_numeric() {
            right -= 1;
        } else {
            left += 1;
        }
    }
    if first_int.is_none() || second_int.is_none() {
        panic!("There are no numbers in the dirty text: [{dirty_text}]")
    }
    (first_int.unwrap(), second_int.unwrap())
}

fn sum_calibrations(input: Vec<(i32, i32)>) -> i32 {
    let mut result = 0;
    for (first, second) in input {
        result += first * 10 + second;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::clean_calibration;
    use crate::clean_calibrations;
    use crate::sum_calibrations;

    #[test]
    fn basic_test() {
        assert_eq!(clean_calibration("1abc2"), (1, 2));
        assert_eq!(clean_calibration("pqr3stu8vwx"), (3, 8));
        assert_eq!(clean_calibration("a1b2c3d4e5f"), (1, 5));
        assert_eq!(clean_calibration("treb7uchet"), (7, 7));
        assert_eq!(clean_calibration("fourvninelccgtkjzhhdqjmnxjbbkdsnine6two"), (6, 6));
        assert_eq!(clean_calibration("12"), (1, 2));
        assert_eq!(clean_calibration("1"), (1, 1));
        assert_eq!(clean_calibration("a1a"), (1, 1));
        assert_eq!(clean_calibration("6three16"), (6, 6));
        assert_eq!(clean_calibration("onekxnsfour7"), (7, 7));
        assert_eq!(clean_calibration("szseight88fourlfcvbzmone1dnzbnkq"), (8, 1));
        assert_eq!(clean_calibration("two75"), (7, 5));
        assert_eq!(clean_calibration("75two"), (7, 5));
        assert_eq!(clean_calibration("two75"), (7, 5));
        assert_eq!(clean_calibration("threerznlrhtkjp23mtflmbrzq395three"), (2, 5));
        let input = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];
        assert_eq!(clean_calibrations(&input), vec!((1, 2), (3, 8), (1, 5), (7, 7)));
        assert_eq!(sum_calibrations(clean_calibrations(&input)), 142)
    }
}