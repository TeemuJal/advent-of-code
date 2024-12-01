use std::{fs::read, io::BufRead};

#[derive(Debug)]
struct SetCubes {
    reds: usize,
    greens: usize,
    blues: usize,
}

struct GameMaxCubes {
    max_reds: usize,
    max_greens: usize,
    max_blues: usize,
}

impl GameMaxCubes {
    fn update_max_vals(&mut self, set_cubes: SetCubes) {
        let SetCubes {
            reds,
            greens,
            blues,
        } = set_cubes;
        if reds > self.max_reds {
            self.max_reds = reds;
        }
        if greens > self.max_greens {
            self.max_greens = greens;
        }
        if blues > self.max_blues {
            self.max_blues = blues;
        }
    }
}

impl SetCubes {
    fn add_reds(&mut self, reds: usize) {
        self.reds += reds;
    }
    fn add_greens(&mut self, greens: usize) {
        self.greens += greens;
    }
    fn add_blues(&mut self, blues: usize) {
        self.blues += blues;
    }
    fn all_fields_less_than_or_equal(&self, other: SetCubes) -> bool {
        return self.reds <= other.reds && self.greens <= other.greens && self.blues <= other.blues;
    }
}

const SET_CUBES_LIMIT: SetCubes = SetCubes {
    reds: 12,
    greens: 13,
    blues: 14,
};

type GamePossible = bool;
type GamePower = usize;
type GameId = usize;
type GameSets<'a> = Vec<&'a str>;

fn main() {
    let input = read("input.txt").unwrap();

    let (part_one_result, part_two_result) = process_games(input);

    println!("Part one result is {part_one_result}");
    println!("Part two result is {part_two_result}");
}

fn process_games(input: Vec<u8>) -> (usize, usize) {
    let mut part_one_result: usize = 0;
    let mut part_two_result: usize = 0;

    for game in input.lines() {
        let game = game.unwrap();
        let game_str = game.as_str();

        let (id, sets) = parse_game_string(game_str);

        let (game_possible, game_power) = process_game_sets(sets);

        if game_possible {
            part_one_result += id;
        }
        part_two_result += game_power;
    }
    return (part_one_result, part_two_result);
}

fn parse_game_string(game: &str) -> (GameId, GameSets) {
    let split: Vec<&str> = game.split(":").collect();

    assert!(split.len() == 2);

    let game_string = *split.get(0).unwrap();
    let cubes = *split.get(1).unwrap();

    let game_string_split: Vec<&str> = game_string.split(" ").collect();
    assert!(game_string_split.len() == 2);
    let id: usize = game_string_split.get(1).unwrap().parse().unwrap();

    let sets: Vec<&str> = cubes.split(";").collect();

    return (id, sets);
}

fn process_game_sets(sets: Vec<&str>) -> (GamePossible, GamePower) {
    let mut game_max_cubes = GameMaxCubes {
        max_reds: 0,
        max_greens: 0,
        max_blues: 0,
    };
    let mut game_possible = true;

    for set in sets {
        let mut set_cubes = SetCubes {
            reds: 0,
            greens: 0,
            blues: 0,
        };
        let cube_strings: Vec<&str> = set.split(",").collect();
        process_cube_strings(cube_strings, &mut set_cubes);

        if !set_cubes.all_fields_less_than_or_equal(SET_CUBES_LIMIT) {
            game_possible = false;
        }
        game_max_cubes.update_max_vals(set_cubes);
    }
    let game_power = game_max_cubes.max_reds * game_max_cubes.max_greens * game_max_cubes.max_blues;

    return (game_possible, game_power);
}

fn process_cube_strings(cube_strings: Vec<&str>, set_cubes: &mut SetCubes) {
    for cube_str in cube_strings {
        let cube_str = cube_str.trim();
        let cube_str_split: Vec<&str> = cube_str.split(" ").collect();
        assert!(cube_str_split.len() == 2);

        let count: usize = cube_str_split.first().unwrap().parse().unwrap();

        let color = *cube_str_split.get(1).unwrap();
        match color {
            "red" => set_cubes.add_reds(count),
            "green" => set_cubes.add_greens(count),
            "blue" => set_cubes.add_blues(count),
            unknown_color => panic!("found unknown color {unknown_color}"),
        }
    }
}
