use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let sum_of_predictions: isize = input.lines().map(parse_line).map(get_prediction).sum();
    println!("Sum of predictions is {sum_of_predictions}");
}

fn get_prediction(values: Vec<isize>) -> isize {
    if values.iter().all(|val| *val == 0) {
        return 0;
    }
    let mut differences: Vec<isize> = Vec::new();
    for (idx, val) in values.iter().enumerate().take(values.len() - 1) {
        differences.push(values[idx + 1] - val);
    }
    let prediction = *values.last().unwrap() + get_prediction(differences);
    return prediction;
}

fn parse_line(str: &str) -> Vec<isize> {
    return str
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect();
}
