use aoc_input::*;
use std::collections::HashMap;
use std::str::FromStr;

enum Instruction {
    Mask(String),
    Assignment(u64, u64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        if s.contains("mem") {
            let left_clause = split.next().unwrap();
            let address = left_clause
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .parse::<u64>().unwrap();
            let right_clase = split.nth(1).unwrap();
            let value = right_clase.parse::<u64>().unwrap();
            Ok(Instruction::Assignment(address, value))
        } else if s.contains("mask") {
            Ok(Instruction::Mask(split.nth(2).unwrap().to_string()))
        } else {
            Err("Not a valid line.".to_string())
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn run_mask_val(instructions: &[Instruction]) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    let mut mask = &"".to_string();
    for inst in instructions {
        match inst {
            Instruction::Mask(new_mask) => mask = new_mask,
            Instruction::Assignment(dest, val) => {map.insert(*dest, apply_mask(*val, mask));},
        };
    };
    map
}

fn apply_mask(num: u64, mask: &str) -> u64 {
    mask.chars().rev().enumerate().fold(num, |acc, (place, chr)| {
        match chr.to_digit(10) {
            Some(0) => acc & !(1 << place),
            Some(1) => acc | (1 << place),
            _ => acc,
        }
    })
}

fn run_mask_address(instructions: &[Instruction]) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    let mut mask = &"".to_string();
    for inst in instructions {
        match inst {
            Instruction::Mask(new_mask) => mask = new_mask,
            Instruction::Assignment(dest, val) => {
                for submask in apply_mask_2(*dest, mask) {
                    map.insert(submask, *val);
                }
            },
        };
    };
    map
}

fn apply_mask_2(mut num: u64, mask: &str) -> Vec<u64> {
    for (place, chr) in mask.chars().rev().enumerate() {
        if chr == '1'{
            num |= 1 << place;
        }
    };
    let mut results: Vec<_> = vec![num];
    for (place, chr) in mask.chars().rev().enumerate() {
        let mut new_zeroes = Vec::new();
        if chr == 'X'{
            for result in results.iter_mut() {
                new_zeroes.push(*result & !(1 << place));
                *result |= 1 << place;
            }
        }
        results.append(&mut new_zeroes);
    };
    let formatted = results.iter().map(|x| format!("{:b}", *x)).collect::<Vec<_>>();
    results
}

fn main() {
    let input = get_input_txt();
    let insts = parse(&input);
    let map = run_mask_val(&insts);
    let sum: u64 = map.values().sum();
    println!("Part one: {}", sum);
    let map2 = run_mask_address(&insts);
    let sum2: u64 = map2.values().sum();
    println!("Part two: {}", sum2);
}

#[test]
fn example_input_1() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    let insts = parse(input);
    let map = run_mask_val(&insts);
    let sum: u64 = map.values().sum();
    assert_eq!(165, sum)
}

#[test]
fn example_input_2() {
    let input = "mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1";
    let insts = parse(input);
    let map = run_mask_address(&insts);
    println!("{:?}", map);
    let sum: u64 = map.values().sum();
    assert_eq!(208, sum);
    assert!(map.contains_key(&59));
    assert!(!map.contains_key(&57));
}
