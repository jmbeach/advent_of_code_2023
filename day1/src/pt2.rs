fn clean_calibration_pt2(dirty_text: &str) -> (i32, i32) {
    let mut first_int: Option<i32> = None;
    let mut second_int: Option<i32> = None;
    let mut text: String = String::from("");
    for char in dirty_text.chars() {
        match char {
            '0'..='9' => {
                text = String::from("");
                let as_int = char.to_string().parse::<i32>().expect("Could not convert char to int");
                if first_int.is_none() {
                    first_int = Some(as_int)
                } else {
                    second_int = Some(as_int)
                }
            }
            _ => {
                text += char.to_string().as_str();
                let text_as_int: Option<i32> = match text.as_str() {
                    "eight" => Some(8),
                    "five" => Some(5),
                    "four" => Some(4),
                    "nine" => Some(9),
                    "one" => Some(1),
                    "seven" => Some(7),
                    "six" => Some(6),
                    "three" => Some(3),
                    "two" => Some(2),
                    "zero" => Some(0),
                    _ => None
                };
                if text_as_int.is_none() {
                    continue
                }
                if first_int.is_none() {
                    first_int = text_as_int
                } else {
                    second_int = text_as_int
                }
            }
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

#[cfg(test)]
mod tests {
    use crate::pt2::clean_calibration_pt2;

    #[test]
    fn basic_test() {
        let input = vec!(
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen"
        );
        assert_eq!(clean_calibration_pt2("two1nine"), (2, 9));
        assert_eq!(clean_calibration_pt2("eightwothree"), (8, 3));
        assert_eq!(clean_calibration_pt2("abcone2threexyz"), (1, 9));
        assert_eq!(clean_calibration_pt2("xtwone3four"), (2, 9));
        assert_eq!(clean_calibration_pt2("4nineeightseven2"), (2, 9));
        assert_eq!(clean_calibration_pt2("zoneight234"), (2, 9));
        assert_eq!(clean_calibration_pt2("7pqrstsixteen"), (2, 9));
    }
}