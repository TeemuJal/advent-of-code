use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let instructions = *lines.first().unwrap();
    let mut nodes_map: HashMap<String, Node> = HashMap::new();
    let nodes: Vec<Node> = lines[2..].iter().map(parse_str_to_node).collect();
    nodes.iter().for_each(|node| {
        nodes_map.insert(node.id.clone(), node.clone());
    });
    let start_nodes: Vec<&Node> = nodes
        .iter()
        .filter(|node| (**node).id.ends_with('A'))
        .collect();
    let counts: Vec<_> = start_nodes.iter().map(|node| {
        let mut current = node.id.as_str();
        let mut step_count: u64 = 0;
        'main_loop: loop {
            for char in instructions.chars() {
                if current.ends_with('Z') {
                    break 'main_loop;
                }
                step_count += 1;
                if char == 'L' {
                    current = &nodes_map.get(current).unwrap().left;
                } else {
                    current = &nodes_map.get(current).unwrap().right;
                }
            }
        }
        step_count
    }).collect();
    let step_count = lcm(counts.as_slice());
    println!("Step count {step_count}");
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    return a * b / gcd(a, b);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
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
