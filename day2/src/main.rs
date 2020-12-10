use aoc_input::{get_argument_parsed, get_input_txt};
use std::io;

type Condition = &'static dyn Fn(&str) -> bool;

fn main() {
    let raw_content = get_input_txt();
    let lines = raw_content.lines();
    let condition: Condition = match get_argument_parsed(1) {
        Some(0) => &part_one,
        Some(1) => &part_two,
        Some(_) => &part_one,
        None => &part_one,
    };
    println!("{}", get_valid_passwords_count(lines, condition));
}

fn get_valid_passwords_count(lines: std::str::Lines, condition: Condition) -> usize {
    let mut valid: i64 = 0;
    lines.filter(|line| condition(line)).count()
}

fn part_one(input: &str) -> bool {
    let split_line: Vec<&str> = input.split(&['-', ' ', ':'][..]).collect();
    if split_line.len() != 5 {
        panic!("Failed to parse line")
    }
    if let [min, max, character, _, target] = split_line[..] {
        let min = min.parse::<usize>().unwrap();
        let max = max.parse::<usize>().unwrap();
        return target.matches(character).count() >= min
            && target.matches(character).count() <= max;
    }
    return false;
}

fn part_two(input: &str) -> bool {
    let split_line: Vec<&str> = input.split(&['-', ' ', ':'][..]).collect();
    if split_line.len() != 5 {
        panic!("Failed to parse line")
    }
    if let [min, max, character, _, target] = split_line[..] {
        let min = min.parse::<usize>().unwrap();
        let max = max.parse::<usize>().unwrap();
        return (match_at(target, character, min) || match_at(target, character, max))
            && !(match_at(target, character, min) && match_at(target, character, max));
    }
    return false;
}

fn match_at(input: &str, pat: &str, target_index: usize) -> bool {
    input
        .match_indices(pat)
        .any(|(index, _)| index + 1 == target_index)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn with_sample_input() {
        let sample: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(2, get_valid_passwords_count(sample.lines(), &part_one))
    }
}
