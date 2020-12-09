use std::fs;
use std::str::{FromStr};

pub fn get_input_txt() -> String {
    fs::read_to_string("input.txt").expect("Can't read input.txt")
}

pub fn get_argument_parsed<T>(pos: usize) -> Option<T> where T: FromStr, <T as FromStr>::Err: std::fmt::Debug {
    let args: Vec<String> = std::env::args().collect();
    let raw_value = args.get(pos)?;
    match raw_value.parse::<T>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}