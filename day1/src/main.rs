use std::collections::HashMap;
use std::fs;

fn main() {
    let digit_str_to_digit = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let input = fs::read_to_string("input.txt").unwrap();
    let mut result: u32 = 0;

    for line in input.lines() {
        let mut first_digit: Option<String> = None;
        let mut last_digit: Option<String> = None;
        let mut digit_string: String = "".to_string();

        for char in line.chars() {
            match char.to_digit(10) {
                Some(digit) => {
                    if first_digit == None {
                        first_digit = Some(digit.to_string());
                    }
                    last_digit = Some(digit.to_string());
                    digit_string = "".to_string();
                    continue;
                }
                _ => digit_string.push(char),
            }
            let mut last_digit_str = "";
            let mut last_digit_str_pos: Option<usize> = None;

            for digit_str in digit_str_to_digit.keys() {
                let digit_string_reversed: String = digit_string.chars().rev().collect();
                let digit_str_reversed: String = digit_str.chars().rev().collect();

                if let Some(pos) = digit_string_reversed.find(&digit_str_reversed) {
                    if let Some(last_pos) = last_digit_str_pos {
                        if pos < last_pos {
                            last_digit_str = digit_str;
                            last_digit_str_pos = Some(pos);
                        }
                    } else {
                        last_digit_str = digit_str;
                        last_digit_str_pos = Some(pos);
                    }
                }
            }
            if last_digit_str != "" {
                let digit = Some(String::from(
                    *digit_str_to_digit.get(last_digit_str).unwrap(),
                ));
                if first_digit == None {
                    first_digit = digit.clone();
                }
                last_digit = digit;
            }
        }
        let mut calibration_value: String = first_digit.unwrap();
        let last_digit_str: String = last_digit.unwrap();
        calibration_value.push_str(last_digit_str.as_str());

        let calibration_value: u32 = calibration_value.parse().unwrap();

        result += calibration_value;
    }
    println!("{result}");
}
