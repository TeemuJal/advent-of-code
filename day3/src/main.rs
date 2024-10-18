use std::fs;

#[derive(Debug)]
struct SchematicNumber {
    number: usize,
    start_index: usize,
    end_index: usize,
}

#[derive(Debug)]
struct SchematicLine {
    numbers: Vec<SchematicNumber>,
    symbol_indices: Vec<usize>,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    //let line_len = input.lines().take(1).collect::<String>().len();
    parse_engine_schematic(input);
}

fn parse_engine_schematic(input: String) -> Vec<SchematicLine> {
    let lines: Vec<SchematicLine> = input
        .lines()
        .map(|line| parse_schematic_line(line))
        .collect();
    return lines;
}

fn parse_schematic_line(og_line: &str) -> SchematicLine {
    let mut schematic_line = SchematicLine {
        numbers: vec![],
        symbol_indices: vec![],
    };
    let mut line = og_line;
    while line != "" {
        println!("line: {line}");
        if let Some(next_char) = line.chars().next() {
            let next_char_index = og_line.len() - line.len();
            match next_char {
                '.' => {
                    line = &line[1..];
                }
                _digit if next_char.is_digit(10) => {
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
                    schematic_line.numbers.push(schematic_number);
                    line = &line[end_index + 1..];
                }
                _ => {
                    schematic_line.symbol_indices.push(next_char_index);
                    line = &line[1..];
                }
            }
        }
    }
    println!("{:?}", schematic_line);
    return schematic_line;
}
