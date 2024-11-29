use std::fs::read_to_string;

#[derive(Debug)]
struct Galaxy {
    id: usize,
    row: usize,
    idx: usize,
}

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let line_len = input.lines().nth(0).unwrap().len();
    let additional_empty_row_col_size = 999999;
    let mut empty_row_indices: Vec<usize> = vec![];
    let mut empty_column_indices: Vec<usize> = vec![];
    let mut universe_rows: Vec<String> = vec![];
    for (idx, line) in input.lines().enumerate() {
        universe_rows.push(line.to_string());
        if line.chars().all(|char| char == '.') {
            empty_row_indices.push(idx);
        }
    }
    for idx in 0..line_len {
        if universe_rows
            .iter()
            .all(|row| row.chars().nth(idx).unwrap() == '.')
        {
            empty_column_indices.push(idx);
        }
    }
    let mut galaxies: Vec<Galaxy> = vec![];
    for (row_idx, row) in universe_rows.iter().enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            if char == '#' {
                galaxies.push(Galaxy {
                    id: galaxies.len(),
                    row: row_idx,
                    idx: col_idx,
                });
            }
        }
    }
    let mut galaxy_pairs: Vec<(&Galaxy, &Galaxy)> = vec![];
    for galaxy in &galaxies {
        for other_galaxy in &galaxies {
            let pair_doesnt_exist = galaxy_pairs
                .iter()
                .find(|(a, b)| a.id == other_galaxy.id && b.id == galaxy.id)
                .is_none();
            if other_galaxy.id != galaxy.id && pair_doesnt_exist {
                galaxy_pairs.push((&galaxy, &other_galaxy));
            }
        }
    }
    let sum_of_shortest_paths: usize = galaxy_pairs
        .iter()
        .map(|(galaxy1, galaxy2)| {
            let x_steps = galaxy1.idx.abs_diff(galaxy2.idx);
            let empty_col_count = empty_column_indices
                .iter()
                .filter(|col_idx| {
                    if galaxy2.idx > galaxy1.idx {
                        **col_idx > galaxy1.idx && **col_idx < galaxy2.idx
                    } else {
                        **col_idx > galaxy2.idx && **col_idx < galaxy1.idx
                    }
                })
                .count();
            let y_steps = galaxy1.row.abs_diff(galaxy2.row);
            let empty_row_count = empty_row_indices
                .iter()
                .filter(|row_idx| {
                    if galaxy2.row > galaxy1.row {
                        **row_idx > galaxy1.row && **row_idx < galaxy2.row
                    } else {
                        **row_idx > galaxy2.row && **row_idx < galaxy1.row
                    }
                })
                .count();
            x_steps
                + y_steps
                + (empty_col_count * additional_empty_row_col_size)
                + (empty_row_count * additional_empty_row_col_size)
        })
        .sum();
    println!(
        "Sum of shortest path lengths between galaxy pairs {}",
        sum_of_shortest_paths
    );
}
