use aoc_input::*;

fn main() {
    let input = get_input_txt();
    let nums = input.lines().map(|x| x.parse::<i64>().unwrap());
    let nums: Vec<_> = nums.collect();
    let len = nums.len();
    let previous = 25;
    let mut invalid = -1;
    for i in previous..nums.len() {
        if !has_sums(&nums, i, previous) {
            invalid = nums[i];
            println!("Found first: {}", invalid);
            break;
        }
    }
    println!("{}", get_range_pair_sum(&nums, invalid).expect("oop"));
}

fn has_sums(nums: &[i64], index: usize, previous: usize) -> bool{
    for i in (index-previous)..index {
        let diff = nums[index] - nums[i];
        if nums[i..index].contains(&diff) {
            return true;
        }
    }
    false
}

fn get_range_pair_sum(nums: &[i64], target: i64) -> Option<i64> {
    let len = nums.len();
    for i in 0..len {
        let mut j = i;
        let mut sum = nums[i];
        let mut smallest = nums[i];
        let mut largest = nums[i];
        loop {
            if sum > target {
                break;
            }
            j += 1;
            sum += nums[j];
            if nums[j] > largest {
                largest = nums[j];
            } else if nums[j] < smallest {
                smallest = nums[j];
            }
            if sum == target {
                return Some(largest + smallest);
            }
        }
    }
    None
}
