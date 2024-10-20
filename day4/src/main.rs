use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let total_points: usize = input
        .lines()
        .map(|line| {
            let (winning_numbers, found_numbers) = parse_line(line);
            get_line_points(winning_numbers, found_numbers)
        })
        .sum();
    println!("Total scratchcard points: {total_points}");
}

fn get_line_points(winning_numbers: Vec<u8>, found_numbers: Vec<u8>) -> usize {
    let matches = (found_numbers
        .iter()
        .filter(|number| winning_numbers.contains(number))
        .collect::<Vec<&u8>>())
    .len();
    if matches == 0 {
        return 0;
    }
    return usize::pow(2, (matches - 1).try_into().unwrap());
}

fn parse_line(line: &str) -> (Vec<u8>, Vec<u8>) {
    let numbers = line.split_once(":").unwrap().1;
    let numbers_split = numbers.split_once("|").unwrap();
    let winning_numbers_str = numbers_split.0.trim();
    let found_numbers_str = numbers_split.1.trim();
    let winning_numbers = numbers_str_to_parsed_vec(winning_numbers_str);
    let found_numbers = numbers_str_to_parsed_vec(found_numbers_str);
    return (winning_numbers, found_numbers);
}

fn numbers_str_to_parsed_vec(numbers_str: &str) -> Vec<u8> {
    return numbers_str
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .collect();
}
