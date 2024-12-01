use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut card_counts: HashMap<usize, usize> = HashMap::new();
    let total_points: usize = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let card_number = idx + 1;
            card_counts
                .entry(card_number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            let card_count = card_counts[&card_number];
            let (winning_numbers, found_numbers) = parse_line(line);
            let line_matches = get_line_matches(winning_numbers, found_numbers);
            for (idx, _) in vec![0; line_matches].iter().enumerate() {
                let card_number_to_update = card_number + idx + 1;
                card_counts
                    .entry(card_number_to_update)
                    .and_modify(|count| *count += card_count)
                    .or_insert(card_count);
            }
            get_line_points(line_matches)
        })
        .sum();
    println!("Total scratchcard points: {total_points}");
    let total_cards: usize = card_counts.values().sum();
    println!("Total number of scratchcards: {total_cards}");
}

fn get_line_matches(winning_numbers: Vec<u8>, found_numbers: Vec<u8>) -> usize {
    return (found_numbers
        .iter()
        .filter(|number| winning_numbers.contains(number))
        .collect::<Vec<&u8>>())
    .len();
}

fn get_line_points(matches: usize) -> usize {
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
