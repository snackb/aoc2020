use std::str::{FromStr, SplitWhitespace};

use aoc_input::*;

fn main() {
    let input = get_input_txt();
    let mut instructions = parse_input(&input);
    for i in 0..instructions.len() {
        swap(&mut instructions, i);
        if let Some(acc) = run_instructions(&mut instructions) {
            println!("{}", acc);
        }
        swap(&mut instructions, i);
    }
}

fn swap(instructions: &mut [(Op, i32, bool)], index: usize) {
    match instructions[index] {
        (Op::Jmp, _, _) => instructions[index].0 = Op::Nop,
        (Op::Nop, _, _) => instructions[index].0 = Op::Jmp,
        _ => (),
    }
}

fn run_instructions(instructions: &mut [(Op, i32, bool)]) -> Option<i32> {
    let mut instructions = instructions.to_vec();
    let mut acc = 0;
    let mut index = 0;
    loop {
        if index == instructions.len() {
            return Some(acc);
        }
        let oldindex = index;
        match &instructions[index] {
            (_, _, true) => return None,
            (Op::Acc, value, _) => acc_op(&mut acc, *value, &mut index),
            (Op::Jmp, value, _) => jmp(&mut index, *value),
            (Op::Nop, _, _) => nop(&mut index),
        };
        instructions[oldindex].2 = true;
    };
}

fn acc_op(acc: &mut i32, value: i32, index: &mut usize) {
    *acc += value; *index += 1;
}

fn jmp(index: &mut usize, value: i32) {
    if value.is_negative() {
        *index -= value.abs() as usize
    } else {
        *index += value as usize
    }
}

fn nop(index: &mut usize) {
    *index += 1
}

#[derive(Debug, Clone)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Op::Acc),
            "jmp" => Ok(Op::Jmp),
            "nop" => Ok(Op::Nop),
            other => Err(format!("{} is not an operation", other)),
        }
    }
}


fn parse_input(input: &str) -> Vec<(Op, i32, bool)> {
    let parse_tokens = |mut x: SplitWhitespace| {
        let op = x.next().unwrap().parse().unwrap();
        let value = x.next().unwrap().parse().unwrap();
        let visited = false;
        (
            op,
            value,
            visited,
        )
    };
    input.lines()
        .map(|line| line.split_whitespace())
        .map(&parse_tokens).collect()
}
