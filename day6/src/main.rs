use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    assert_eq!(lines.len(), 2);

    let durations_str = *lines.first().unwrap();
    let durations = parse_string_to_integer_vec(durations_str);
    let record_distances_str = *lines.last().unwrap();
    let record_distances = parse_string_to_integer_vec(record_distances_str);
    assert_eq!(durations.len(), record_distances.len());

    let error_margins: Vec<usize> = durations.iter().enumerate().map(|(idx, duration)| {
        let duration = *duration;
        let record_distance = *record_distances.get(idx).unwrap();
        (1..duration)
            .map(|time| time * (duration - time))
            .filter(|distance| *distance > record_distance)
            .count()
    }).collect();

    let part_one_result = error_margins.iter().fold(1, |acc, margin| acc * margin);
    println!("Part one result is {part_one_result}");
}

fn parse_string_to_integer_vec(str: &str) -> Vec<usize> {
    return str
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .collect();
}