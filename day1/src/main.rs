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
    return result
}

fn clean_calibration(dirty_text: &str) -> (i32, i32) {
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
        let input = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet"
        ];
        assert_eq!(clean_calibrations(&input), vec!((1, 2), (3, 8), (1, 5), (7, 7)));
        assert_eq!(sum_calibrations(clean_calibrations(&input)), 142)
    }
}