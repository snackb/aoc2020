use aoc_input::*;
use std::collections::HashMap;

fn get_num_at(mut starters: &[u64], count: u64) -> u64 {
    let mut prev = None;
    let mut map = HashMap::new();
    for i in 1..=count {
        let cur = if !starters.is_empty() {
            let tmp = starters[0];
            starters = &starters[1..];
            tmp
        } else if let Some(turn) = map.get(&prev.unwrap()) {
            i - *turn
        } else {
            0
        };
        if let Some(num ) = prev {
            map.insert(num, i);
        };
        prev = Some(cur);
    };
    prev.unwrap()
}

fn parse(input :&str) -> Vec<u64> {
    input.split(',').map(|x|x.parse().unwrap()).collect()
}

fn main() {
    let input = get_input_txt();
    let nums = parse(&input);
    println!("part one: {}", get_num_at(&nums, 2020));
    println!("part two: {}", get_num_at(&nums, 30000000));
}

#[test]
fn example_input_1() {
    let input = "0,3,6";
    let nums = parse(input);
    assert_eq!(436, get_num_at(&nums, 2020))
}