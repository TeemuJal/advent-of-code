use std::{collections::HashMap, fs::read_to_string};

struct Node {
    id: String,
    left: String,
    right: String,
}

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let instructions = *lines.first().unwrap();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    lines[2..]
        .iter()
        .map(parse_str_to_node)
        .for_each(|node| {
            nodes.insert(node.id.clone(), node);
        });
    let mut step_count = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let node = nodes.get(current).unwrap();
        for char in instructions.chars() {
            step_count += 1;
            if char == 'L' {
                current = &nodes.get(&node.left).unwrap().id;
            } else {
                current = &nodes.get(&node.right).unwrap().id;
            }
        }
    }
    println!("Step count {step_count}");
}

fn parse_str_to_node(str: &&str) -> Node {
    let str = *str;
    let split = str.split_once("=").unwrap();
    let id = split.0.trim().to_string();
    let left_right = split.1.trim();
    let left = &left_right[1..=3];
    let right = &left_right[6..=8];
    return Node {
        id,
        left: left.to_string(),
        right: right.to_string(),
    };
}
