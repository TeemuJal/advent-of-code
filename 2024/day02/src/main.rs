use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let safe_report_count = input
        .lines()
        .filter(|line| {
            let levels: Vec<usize> = line
                .split_whitespace()
                .map(|nr| nr.parse().unwrap())
                .collect();
            let mut pair_iter = levels[0..levels.len() - 1].iter().zip(levels[1..].iter());
            let all_increasing_within_limits = pair_iter.clone().all(|(a, b)| {
                a.checked_sub(*b).is_none() && (a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3)
            });
            if all_increasing_within_limits {
                return true;
            }
            let all_decreasing_within_limits = pair_iter.all(|(a, b)| {
                b.checked_sub(*a).is_none() && (a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3)
            });
            all_decreasing_within_limits
        })
        .count();
    println!("Number of safe reports is {safe_report_count}");
}
