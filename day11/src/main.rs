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
    let empty_row = '.'.to_string().repeat(line_len);
    let mut expanded_universe_rows: Vec<String> = vec![];
    for line in input.lines() {
        expanded_universe_rows.push(line.to_string());
        if line.chars().all(|char| char == '.') {
            expanded_universe_rows.push(empty_row.clone());
        }
    }
    let mut columns_added: usize = 0;
    for idx in 0..line_len {
        let current_idx = idx + columns_added;
        if expanded_universe_rows
            .iter()
            .all(|row| row.chars().nth(current_idx).unwrap() == '.')
        {
            expanded_universe_rows
                .iter_mut()
                .for_each(|row| row.insert(current_idx + 1, '.'));
            columns_added += 1;
        }
    }
    let mut galaxies: Vec<Galaxy> = vec![];
    for (row_idx, row) in expanded_universe_rows.iter().enumerate() {
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
            let y_steps = galaxy1.row.abs_diff(galaxy2.row);
            x_steps + y_steps
        })
        .sum();
    println!(
        "Sum of shortest path lengths between galaxy pairs {}",
        sum_of_shortest_paths
    );
}
