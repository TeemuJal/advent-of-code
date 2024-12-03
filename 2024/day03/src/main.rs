use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>().join("");
    let mut split: Vec<_> = input.split("mul(").collect();
    split.remove(0);
    let mut multiplications_sum = 0;
    for str in split {
        let split = str.split_once(',');
        if let Some((number1, rest)) = split {
            if number1.len() > 3 {
                continue;
            }
            if let Ok(number1) = number1.parse::<usize>() {
                let split = rest.split_once(')');
                if let Some((number2, _)) = split {
                    if number2.len() > 3 {
                        continue;
                    }
                    if let Ok(number2) = number2.parse::<usize>() {
                        multiplications_sum += number1 * number2;
                    }
                }
            }
        }
    }
    println!("Sum of multiplications {multiplications_sum}");
}
