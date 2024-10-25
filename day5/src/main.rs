use std::fs::read_to_string;

type DestinationRangeStart = i64;
type SourceRangeStart = i64;
type RangeLength = i64;
type MapEntry = (DestinationRangeStart, SourceRangeStart, RangeLength);

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input file");
    let mut lines = input.lines().peekable();
    let seeds_str: Vec<&str> = lines.by_ref().take_while(|line| *line != "").collect();
    assert_eq!(seeds_str.len(), 1);
    let seeds_str = seeds_str[0];
    let seed_numbers = numbers_str_to_parsed_vec(seeds_str.split_once(":").unwrap().1);

    let mut map_vecs: Vec<Vec<&str>> = vec![];
    while lines.peek().is_some() {
        let map_strings: Vec<&str> = lines.by_ref().take_while(|line| *line != "").collect();
        map_vecs.push(map_strings);
    }
    let location_numbers: Vec<i64> = seed_numbers
        .iter()
        .map(|seed_number| seed_number_to_location_number(*seed_number, &map_vecs))
        .collect();
    let min_location_number = location_numbers.iter().min().expect("No location numbers");
    println!("Lowest location number: {min_location_number}");
}

fn seed_number_to_location_number(seed_number: i64, map_vecs: &Vec<Vec<&str>>) -> i64 {
    let mut map_result = seed_number;
    'map_vecs: for map in map_vecs {
        let map = parse_map_vec(map);
        for entry in map {
            let (destination_range_start, source_range_start, range_len) = entry;
            if map_result >= source_range_start && map_result < (source_range_start + range_len) {
                let offset = destination_range_start - source_range_start;
                map_result = map_result + offset;
                continue 'map_vecs;
            }
        }
    }
    return map_result;
}

fn parse_map_vec(map_vec_strings: &Vec<&str>) -> Vec<MapEntry> {
    let map_vec_strings: Vec<&str> = map_vec_strings[1..].to_vec();
    return map_vec_strings.iter().map(parse_map_string).collect();
}

fn parse_map_string(map_string: &&str) -> MapEntry {
    let map_numbers: Vec<i64> = numbers_str_to_parsed_vec(map_string);
    assert_eq!(map_numbers.len(), 3);
    return (map_numbers[0], map_numbers[1], map_numbers[2]);
}

fn numbers_str_to_parsed_vec(numbers_str: &str) -> Vec<i64> {
    return numbers_str
        .split_whitespace()
        .map(|number_str| {
            number_str
                .parse()
                .expect("Couldn't parse string slice to a number")
        })
        .collect();
}
