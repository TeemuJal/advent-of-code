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
    let map_vecs = map_vecs.iter().map(parse_map_vec).collect();

    let part_one_location_numbers: Vec<i64> = seed_numbers
        .iter()
        .map(|seed_number| seed_number_to_location_number(*seed_number, &map_vecs))
        .collect();
    let part_one_min_location_number = part_one_location_numbers.iter().min().unwrap();
    println!("Part one lowest location number: {part_one_min_location_number}");

    let seed_numbers_range_pairs = seed_numbers.chunks(2);
    let part_two_min_location_number = seed_numbers_range_pairs
        .map(|pair| {
            let start = pair[0];
            let end = start + pair[1] - 1;
            let seed_range = (start, end);
            seed_range_to_location_range(seed_range, &map_vecs)
        })
        .flatten()
        .map(|range| range.0)
        .min()
        .unwrap();
    println!("Part two lowest location number: {part_two_min_location_number}");
}

fn seed_range_to_location_range(
    seed_range: (i64, i64),
    map_vecs: &Vec<Vec<MapEntry>>,
) -> Vec<(i64, i64)> {
    let mut ranges_to_process = vec![seed_range];
    let mut map_result_ranges = vec![];
    for map in map_vecs {
        'range_loop: while ranges_to_process.len() > 0 {
            let (range_start, range_end) = ranges_to_process.pop().unwrap();
            for entry in map {
                let (destination_range_start, source_range_start, range_len) = *entry;
                let source_range_end = source_range_start + range_len - 1;
                let offset = destination_range_start - source_range_start;
                if range_start >= source_range_start && range_start <= source_range_end {
                    let result_range_start = range_start + offset;
                    let result_range_end = if range_end <= source_range_end {
                        range_end
                    } else {
                        ranges_to_process.push((source_range_end + 1, range_end));
                        source_range_end
                    };
                    let result_range_end = result_range_end + offset;
                    map_result_ranges.push((result_range_start, result_range_end));
                    continue 'range_loop;
                }
                if range_end <= source_range_end && range_end >= source_range_start {
                    let result_range_end = range_end + offset;
                    let result_range_start = source_range_start + offset;
                    map_result_ranges.push((result_range_start, result_range_end));
                    ranges_to_process.push((range_start, source_range_start - 1));
                    continue 'range_loop;
                }
            }
            map_result_ranges.push((range_start, range_end));
        }
        ranges_to_process = map_result_ranges.clone();
        map_result_ranges = vec![];
    }
    return ranges_to_process;
}

fn seed_number_to_location_number(seed_number: i64, map_vecs: &Vec<Vec<MapEntry>>) -> i64 {
    let mut map_result = seed_number;
    'map_vecs: for map in map_vecs {
        for entry in map {
            let (destination_range_start, source_range_start, range_len) = entry;
            if map_result >= *source_range_start && map_result < (source_range_start + range_len) {
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
