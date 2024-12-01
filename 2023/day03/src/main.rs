use std::fs;

#[derive(Debug)]
struct SchematicNumber {
    number: usize,
    start_index: usize,
    end_index: usize,
}

impl SchematicNumber {
    fn is_part_number(
        &self,
        prev_line: Option<&SchematicLine>,
        current_line_symbol_indices: &Vec<usize>,
        next_line: Option<&SchematicLine>,
    ) -> bool {
        let start_index = self.start_index;
        let end_index = self.end_index;
        let adjacent_start_index = if start_index > 0 {
            start_index - 1
        } else {
            start_index
        };
        let adjacent_end_index = end_index + 1;

        if let Some(prev_line) = prev_line {
            let is_adjacent_to_prev_line = prev_line
                .symbol_indices
                .iter()
                .any(|idx| *idx >= adjacent_start_index && *idx <= adjacent_end_index);
            if is_adjacent_to_prev_line {
                return true;
            }
        }
        if let Some(next_line) = next_line {
            let is_adjacent_to_next_line = next_line
                .symbol_indices
                .iter()
                .any(|idx| *idx >= adjacent_start_index && *idx <= adjacent_end_index);
            if is_adjacent_to_next_line {
                return true;
            }
        }
        return current_line_symbol_indices.contains(&adjacent_start_index)
            || current_line_symbol_indices.contains(&adjacent_end_index);
    }
}

#[derive(Debug)]
struct SchematicLine {
    schematic_numbers: Vec<SchematicNumber>,
    symbol_indices: Vec<usize>,
    potential_gear_indices: Vec<usize>,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let schematic_lines: Vec<SchematicLine> = input.lines().map(parse_schematic_line).collect();
    let (part_numbers_sum, gear_ratios_sum) =
        get_part_numbers_sum_and_gear_ratios_sum(schematic_lines);
    println!("Sum of all engine schematics part numbers is {part_numbers_sum}");
    println!("Sum of all engine schematics gear ratios is {gear_ratios_sum}");
}

fn parse_schematic_line(og_line: &str) -> SchematicLine {
    let mut schematic_line = SchematicLine {
        schematic_numbers: vec![],
        symbol_indices: vec![],
        potential_gear_indices: vec![],
    };
    let mut line = og_line;
    while line != "" {
        if let Some(next_char) = line.chars().next() {
            let next_char_index = og_line.len() - line.len();
            match next_char {
                '.' => {
                    line = &line[1..];
                }
                _ if next_char.is_digit(10) => {
                    let end_index =
                        if let Some(first_non_digit) = line.find(|c: char| !c.is_digit(10)) {
                            first_non_digit - 1
                        } else {
                            line.len() - 1
                        };
                    let schematic_number = SchematicNumber {
                        number: line[0..=end_index].parse().unwrap(),
                        start_index: next_char_index,
                        end_index: next_char_index + end_index,
                    };
                    schematic_line.schematic_numbers.push(schematic_number);
                    line = &line[end_index + 1..];
                }
                symbol => {
                    schematic_line.symbol_indices.push(next_char_index);
                    if symbol == '*' {
                        schematic_line.potential_gear_indices.push(next_char_index);
                    }
                    line = &line[1..];
                }
            }
        }
    }
    return schematic_line;
}

fn get_part_numbers_sum_and_gear_ratios_sum(schematic_lines: Vec<SchematicLine>) -> (usize, usize) {
    let mut part_numbers_sum: usize = 0;
    let mut gear_ratios_sum: usize = 0;
    for (idx, line) in schematic_lines.iter().enumerate() {
        let SchematicLine {
            schematic_numbers,
            symbol_indices,
            potential_gear_indices,
        } = line;
        let prev_line = if idx > 0 {
            schematic_lines.get(idx - 1)
        } else {
            None
        };
        let next_line = schematic_lines.get(idx + 1);
        for schematic_number in schematic_numbers {
            if schematic_number.is_part_number(prev_line, symbol_indices, next_line) {
                part_numbers_sum += schematic_number.number;
            }
        }
        for potential_gear_index in potential_gear_indices {
            if let Some(gear_ratio) = get_gear_ratio(
                *potential_gear_index,
                schematic_numbers,
                prev_line,
                next_line,
            ) {
                gear_ratios_sum += gear_ratio;
            }
        }
    }
    return (part_numbers_sum, gear_ratios_sum);
}

fn get_gear_ratio(
    potential_gear_index: usize,
    schematic_numbers: &[SchematicNumber],
    prev_line: Option<&SchematicLine>,
    next_line: Option<&SchematicLine>,
) -> Option<usize> {
    let mut adjacent_schematic_number_values = vec![];

    let adjacent_start_index = if potential_gear_index > 0 {
        potential_gear_index - 1
    } else {
        potential_gear_index
    };
    let adjacent_end_index = potential_gear_index + 1;

    for schematic_number in schematic_numbers {
        if is_gear_adjacent_to_schematic_number(
            adjacent_start_index,
            adjacent_end_index,
            schematic_number,
        ) {
            adjacent_schematic_number_values.push(schematic_number.number);
        }
    }
    if let Some(prev_line) = prev_line {
        for schematic_number in prev_line.schematic_numbers.iter() {
            if is_gear_adjacent_to_schematic_number(
                adjacent_start_index,
                adjacent_end_index,
                schematic_number,
            ) {
                adjacent_schematic_number_values.push(schematic_number.number);
            }
        }
    }
    if let Some(next_line) = next_line {
        for schematic_number in next_line.schematic_numbers.iter() {
            if is_gear_adjacent_to_schematic_number(
                adjacent_start_index,
                adjacent_end_index,
                schematic_number,
            ) {
                adjacent_schematic_number_values.push(schematic_number.number);
            }
        }
    }
    if adjacent_schematic_number_values.len() == 2 {
        return Some(adjacent_schematic_number_values[0] * adjacent_schematic_number_values[1]);
    }
    return None;
}

fn is_gear_adjacent_to_schematic_number(
    adjacent_start_index: usize,
    adjacent_end_index: usize,
    schematic_number: &SchematicNumber,
) -> bool {
    let number_start_index = schematic_number.start_index;
    let number_end_index = schematic_number.end_index;
    return adjacent_start_index <= number_end_index && adjacent_end_index >= number_start_index;
}
