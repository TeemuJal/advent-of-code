use std::{fs::read_to_string, iter::zip};

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let mut ids1: Vec<usize> = vec![];
    let mut ids2: Vec<usize> = vec![];
    for line in input.lines() {
        let mut split = line.split_whitespace();
        ids1.push(split.nth(0).unwrap().parse().unwrap());
        ids2.push(split.nth(0).unwrap().parse().unwrap());
    }
    ids1.sort();
    ids2.sort();
    let distance_between_lists: usize = zip(ids1, ids2).map(|(a, b)| a.abs_diff(b)).sum();
    println!("Distance between lists is {distance_between_lists}");
}
