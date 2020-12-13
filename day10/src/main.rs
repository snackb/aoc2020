use std::collections::HashMap;
use std::time::{Instant};
use aoc_input::*;

fn main() {
    let input = get_input_txt();
    let nums = input_to_list(&input);
    let diffs = nums.windows(2)
        .map(|x| {
            x[1]-x[0]
        });
    let ones = diffs.clone().filter(|x|*x==1).count();
    let threes = diffs.clone().filter(|x| *x == 3).count();
    println!("Part one: {}", ones * threes);    

    let then = Instant::now();
    let part_two_result = num_distinct_orders(&nums, &mut HashMap::new());
    println!("{:?}", Instant::now() - then);

    let then = Instant::now();
    let part_two_result_better = num_distinct_orders_better(&nums, &mut HashMap::new());
    println!("{:?}", Instant::now() - then);
    
    println!("Part two: {} {}", part_two_result, part_two_result_better);
}

fn input_to_list(input: &str) -> Vec<i64> {
    let nums = input.lines().map(|x|x.parse::<i64>().unwrap());
    let mut nums = nums.collect::<Vec<_>>();
    nums.push(0);
    nums.sort_unstable();
    nums.push(nums.last().unwrap()+3);
    nums
}

fn num_distinct_orders(nums: &[i64], cache: &mut HashMap<usize, i64>) -> i64 {
    let l = nums.len();
    match cache.get(&l) {
        Some(value) => *value,
        None => {
            let result = match l {
                1 => 1, 
                2 => 1, 
                3 => if nums[2] - nums[0] <= 3 {2} else {1},
                _ => {
                    (2..=4).filter(|x|nums[l-1] - nums[l-*x] <= 3)
                        .map(|x| num_distinct_orders(&nums[..l-(x-1)], cache))
                        .sum()
                }
            };
            cache.insert(nums.len(), result);
            result
        }
    }
}

fn num_distinct_orders_better(nums: &[i64], cache: &mut HashMap<i64, i64>) -> i64 {
    let value = nums[0];
    if value == 0 {
        cache.insert(0, 1);
        num_distinct_orders_better(&nums[1..], cache)
    } else if nums.len() == 1 {
        *cache.get(&(value-3)).unwrap()
    } else {
        cache.insert(value,
            cache.get(&(value - 1)).unwrap_or(&0) + 
            cache.get(&(value-2)).unwrap_or(&0) +
            cache.get(&(value-3)).unwrap_or(&0)
        );
        num_distinct_orders_better(&nums[1..], cache)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        let nums = input_to_list(input);
        assert_eq!(8, num_distinct_orders(&nums, &mut HashMap::new()));
        assert_eq!(8, num_distinct_orders_better(&nums, &mut HashMap::new()));
    }

    #[test]
    fn example2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let nums = input_to_list(input);
        assert_eq!(19208, num_distinct_orders(&nums, &mut HashMap::new()));
        assert_eq!(19208, num_distinct_orders_better(&nums, &mut HashMap::new()));
    }
}

