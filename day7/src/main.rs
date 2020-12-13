use aoc_input::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = get_input_txt();
    let rules = input.lines().map(Rule::new).collect::<Vec<_>>();
    let result = match get_argument_parsed(1) {
        Some(0) => get_num_of_bags_can_hold(&rules, "shiny gold"),
        Some(1) => get_num_bags_held(&rules, "shiny gold"),
        _ => get_num_of_bags_can_hold(&rules, "shiny gold"),
    };
    println!("{:?}", result);
}

fn get_num_of_bags_can_hold(rules: &[Rule], target: &str) -> usize {
    let mut set = HashSet::new();
    set.insert(target.to_string());
    let mut prev_size = 1;
    loop {
        for rule in rules {
            if rule.can_hold.iter().any(|x| set.contains(&x.1)) {
                set.insert(rule.bag_type.clone());
            }
        }
        if set.len() == prev_size {
            println!("{:?}", set);
            return prev_size - 1;
        } else {
            prev_size = set.len()
        }
    }
}

fn get_num_bags_held(rules: &[Rule], target: &str) -> usize {
    let rules_map: HashMap<_, _> = rules
        .iter()
        .map(|x| (x.bag_type.clone(), x.clone()))
        .collect();

    fn get_rules(rules: &HashMap<String, Rule>, rule: Rule) -> Vec<Rule> {
        rule.can_hold
            .into_iter()
            .flat_map(|x| {
                let (num, name) = x;
                let mut new_vec = Vec::new();
                for _ in 0..num {
                    new_vec.push(rules.get(&name).unwrap().clone());
                }
                new_vec
            })
            .collect()
    }

    let target_rule = rules_map.get(target).unwrap().clone();
    let mut layer = vec![target_rule];
    let mut total = 0;

    loop {
        match layer.len() {
            0 => return total - 1,
            x => total += x,
        }
        let next_layer = layer.into_iter().flat_map(|x| get_rules(&rules_map, x));
        layer = next_layer.collect();
    }
}

#[derive(Debug, Clone)]
struct Rule {
    bag_type: String,
    can_hold: Vec<(usize, String)>,
}

impl Rule {
    fn new(line: &str) -> Self {
        let split_line = &line.split("contain").collect::<Vec<_>>();
        let bag_type = split_line[0]
            .trim()
            .trim_end_matches("bags")
            .trim()
            .to_string();
        let bags = split_line[1];
        if !bags.contains("no other bags") {
            let bags = bags
                .split(',')
                .map(|x| {
                    let tokens = x.split_whitespace().collect::<Vec<_>>();
                    (
                        tokens[0].parse::<usize>().unwrap(),
                        tokens[1].to_string() + " " + tokens[2],
                    )
                })
                .collect::<Vec<_>>();
            Rule {
                bag_type,
                can_hold: bags,
            }
        } else {
            Rule {
                bag_type,
                can_hold: Vec::new(),
            }
        }
    }
}
