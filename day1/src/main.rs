use aoc_input::{InputBuilder};
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please provide a target and a number to count");
    }
    let target = args[1].parse::<i64>().expect("Please provide a target number");
    let count = args[2].parse::<i64>().expect("Please provide a target number");
    
    let mut builder = InputBuilder::new();
    builder.file("input", "input.txt");
    let results = builder.get_inputs();

    let raw_content = results.values.get("input").unwrap().as_ref().unwrap();
    let lines = raw_content.lines();
    let ints: Vec<i64> = lines.map(|line| line.parse::<i64>().unwrap()).collect();
    let time = std::time::Instant::now();
    match find_solution(&ints, target, count) {
        Some(number) => println!("Result: {}", number),
        None => println!("No result found"),
    }
    println!("{:?}", time.elapsed());
}

fn find_solution(numbers: &[i64], target: i64, num_to_sum: i64) -> Option<i64> {
    for (index, number) in numbers.iter().enumerate() {
        let number = number.clone();
        if num_to_sum > 1 {
            match find_solution(&numbers[index..], target-number, num_to_sum - 1){
                Some(result) => return Some(result * number),
                None => (),
            }
        }
        else if number.clone() == target {
            return Some(number);
        }
    }
    return None;
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_inline() {
        let values = vec![1721,
            979,
            366,
            299,
            675,
            1456];
        let result = find_solution(&values, 2020, 2).unwrap();
        assert_eq!(514579, result);
    }
}
