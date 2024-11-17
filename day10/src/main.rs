use std::fs::read_to_string;

enum PipePosition {
    TopLeftCorner,
    Top,
    TopRightCorner,
    Right,
    BottomRightCorner,
    Bottom,
    BottomLeftCorner,
    Left,
    Middle,
}

fn main() {
    let input = read_to_string("input3.txt").unwrap();
    let line_len = input.lines().nth(0).unwrap().len();
    let pipes = input.lines().collect::<Vec<&str>>().join("");
    let pipes_len = pipes.len();
    let mut current_idx = pipes.find('S').unwrap();
    let mut previous_idx: Option<usize> = None;
    let mut loop_indices: Vec<usize> = Vec::new();
    loop {
        let pipe_position = get_pipe_position(current_idx, line_len, pipes_len);
        let next_idx = get_next_idx(
            current_idx,
            previous_idx,
            pipe_position,
            pipes.as_str(),
            line_len,
        );
        loop_indices.push(next_idx);
        if pipes.chars().nth(next_idx).unwrap() == 'S' {
            break;
        }
        previous_idx = Some(current_idx);
        current_idx = next_idx;
    }
    let steps_to_furthest_point = loop_indices.len() / 2;
    println!("Steps to furthest point {steps_to_furthest_point}");
}

fn get_pipe_position(current_idx: usize, line_len: usize, pipes_len: usize) -> PipePosition {
    let top_left_corner_idx = 0;
    let top_right_corner_idx = line_len - 1;
    let bottom_right_corner_idx = pipes_len - 1;
    let bottom_left_corner_idx = pipes_len - line_len;
    return match current_idx {
        0 => PipePosition::TopLeftCorner,
        x if x == top_right_corner_idx => PipePosition::TopRightCorner,
        x if x == bottom_right_corner_idx => PipePosition::BottomRightCorner,
        x if x == bottom_left_corner_idx => PipePosition::BottomLeftCorner,
        x if x > top_left_corner_idx && x < top_right_corner_idx => PipePosition::Top,
        x if x % line_len == line_len - 1 => PipePosition::Right,
        x if x > bottom_left_corner_idx && x < bottom_right_corner_idx => PipePosition::Bottom,
        x if x % line_len == 0 => PipePosition::Left,
        _ => PipePosition::Middle,
    };
}

fn get_next_idx(
    current_idx: usize,
    previous_idx: Option<usize>,
    pipe_position: PipePosition,
    pipes: &str,
    line_len: usize,
) -> usize {
    match pipe_position {
        PipePosition::TopLeftCorner => {
            let right_idx = current_idx + 1;
            if should_go_right(pipes, current_idx, previous_idx, right_idx) {
                return right_idx;
            }
            let bottom_idx = current_idx + line_len;
            return bottom_idx;
        }
        PipePosition::Top => {
            let right_idx = current_idx + 1;
            if should_go_right(pipes, current_idx, previous_idx, right_idx) {
                return right_idx;
            }
            let bottom_idx = current_idx + line_len;
            if should_go_bottom(pipes, current_idx, previous_idx, bottom_idx) {
                return bottom_idx;
            }
            let left_idx = current_idx - 1;
            return left_idx;
        }
        PipePosition::TopRightCorner => {
            let left_idx = current_idx - 1;
            if should_go_left(pipes, current_idx, previous_idx, left_idx) {
                return left_idx;
            }
            let bottom_idx = current_idx + line_len;
            return bottom_idx;
        }
        PipePosition::Right => {
            let top_idx = current_idx - line_len;
            if should_go_top(pipes, current_idx, previous_idx, top_idx) {
                return top_idx;
            }
            let bottom_idx = current_idx + line_len;
            if should_go_bottom(pipes, current_idx, previous_idx, bottom_idx) {
                return bottom_idx;
            }
            let left_idx = current_idx - 1;
            return left_idx;
        }
        PipePosition::BottomRightCorner => {
            let top_idx = current_idx - line_len;
            if should_go_top(pipes, current_idx, previous_idx, top_idx) {
                return top_idx;
            }
            let left_idx = current_idx - 1;
            return left_idx;
        }
        PipePosition::Bottom => {
            let top_idx = current_idx - line_len;
            if should_go_top(pipes, current_idx, previous_idx, top_idx) {
                return top_idx;
            }
            let right_idx = current_idx + 1;
            if should_go_right(pipes, current_idx, previous_idx, right_idx) {
                return right_idx;
            }
            let left_idx = current_idx - 1;
            return left_idx;
        }
        PipePosition::BottomLeftCorner => {
            let top_idx = current_idx - line_len;
            if should_go_top(pipes, current_idx, previous_idx, top_idx) {
                return top_idx;
            }
            let right_idx = current_idx + 1;
            return right_idx;
        }
        PipePosition::Left => {
            let top_idx = current_idx - line_len;
            if should_go_top(pipes, current_idx, previous_idx, top_idx) {
                return top_idx;
            }
            let right_idx = current_idx + 1;
            if should_go_right(pipes, current_idx, previous_idx, right_idx) {
                return right_idx;
            }
            let bottom_idx = current_idx + line_len;
            return bottom_idx;
        }
        PipePosition::Middle => {
            let top_idx = current_idx - line_len;
            if should_go_top(pipes, current_idx, previous_idx, top_idx) {
                return top_idx;
            }
            let right_idx = current_idx + 1;
            if should_go_right(pipes, current_idx, previous_idx, right_idx) {
                return right_idx;
            }
            let bottom_idx = current_idx + line_len;
            if should_go_bottom(pipes, current_idx, previous_idx, bottom_idx) {
                return bottom_idx;
            }
            let left_idx = current_idx - 1;
            return left_idx;
        }
    }
}

fn should_go_top(
    pipes: &str,
    current_idx: usize,
    previous_idx: Option<usize>,
    top_idx: usize,
) -> bool {
    let current_char = pipes.chars().nth(current_idx).unwrap();
    let top_char = pipes.chars().nth(top_idx).unwrap();
    if top_connects(top_char, current_char) {
        if previous_idx.is_none() {
            return true;
        }
        if let Some(previous_idx) = previous_idx {
            if previous_idx != top_idx {
                return true;
            }
        }
    }
    return false;
}

fn should_go_right(
    pipes: &str,
    current_idx: usize,
    previous_idx: Option<usize>,
    right_idx: usize,
) -> bool {
    let current_char = pipes.chars().nth(current_idx).unwrap();
    let right_char = pipes.chars().nth(right_idx).unwrap();
    if right_connects(right_char, current_char) {
        if previous_idx.is_none() {
            return true;
        }
        if let Some(previous_idx) = previous_idx {
            if previous_idx != right_idx {
                return true;
            }
        }
    }
    return false;
}

fn should_go_bottom(
    pipes: &str,
    current_idx: usize,
    previous_idx: Option<usize>,
    bottom_idx: usize,
) -> bool {
    let current_char = pipes.chars().nth(current_idx).unwrap();
    let bottom_char = pipes.chars().nth(bottom_idx).unwrap();
    if bottom_connects(bottom_char, current_char) {
        if previous_idx.is_none() {
            return true;
        }
        if let Some(previous_idx) = previous_idx {
            if previous_idx != bottom_idx {
                return true;
            }
        }
    }
    return false;
}

fn should_go_left(
    pipes: &str,
    current_idx: usize,
    previous_idx: Option<usize>,
    left_idx: usize,
) -> bool {
    let current_char = pipes.chars().nth(current_idx).unwrap();
    let left_char = pipes.chars().nth(left_idx).unwrap();
    if left_connects(left_char, current_char) {
        if previous_idx.is_none() {
            return true;
        }
        if let Some(previous_idx) = previous_idx {
            if previous_idx != left_idx {
                return true;
            }
        }
    }
    return false;
}

const CONNECTS_TO_TOP: &str = "|JLS";
const CONNECTS_TO_RIGHT: &str = "-FLS";
const CONNECTS_TO_BOT: &str = "|F7S";
const CONNECTS_TO_LEFT: &str = "-7JS";

fn top_connects(char: char, current_char: char) -> bool {
    if !CONNECTS_TO_TOP.contains(current_char) {
        return false;
    }
    return match char {
        '|' => true,
        '7' => true,
        'F' => true,
        'S' => true,
        _ => false,
    };
}

fn right_connects(char: char, current_char: char) -> bool {
    if !CONNECTS_TO_RIGHT.contains(current_char) {
        return false;
    }
    return match char {
        '-' => true,
        'J' => true,
        '7' => true,
        'S' => true,
        _ => false,
    };
}

fn bottom_connects(char: char, current_char: char) -> bool {
    if !CONNECTS_TO_BOT.contains(current_char) {
        return false;
    }
    return match char {
        '|' => true,
        'J' => true,
        'L' => true,
        'S' => true,
        _ => false,
    };
}

fn left_connects(char: char, current_char: char) -> bool {
    if !CONNECTS_TO_LEFT.contains(current_char) {
        return false;
    }
    return match char {
        '-' => true,
        'L' => true,
        'F' => true,
        'S' => true,
        _ => false,
    };
}
