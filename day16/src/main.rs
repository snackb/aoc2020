#![feature(str_split_once)]

use aoc_input::*;
use std::collections::HashMap;

type Rule = ((u64, u64), (u64, u64));

fn valid_by_rule(rule: &Rule, value: u64) -> bool {
    (rule.0 .0 <= value && rule.0 .1 >= value) || (rule.1 .0 <= value && rule.1 .1 >= value)
}

#[derive(Debug)]
struct Document {
    rules: HashMap<String, Rule>,
    my_ticket: Vec<u64>,
    tickets: Vec<Vec<u64>>,
}

impl Document {
    fn new(input: &str) -> Self {
        fn line_to_rule(line: &str) -> (String, Rule) {
            let (name, conds) = line.split_once(':').unwrap();
            let (left, right) = conds.trim().split_once("or").unwrap();
            let (leftmin, leftmax) = left.trim().split_once('-').unwrap();
            let (rightmin, rightmax) = right.trim().split_once('-').unwrap();
            let conds = (
                (leftmin.parse().unwrap(), leftmax.parse().unwrap()),
                (rightmin.parse().unwrap(), rightmax.parse().unwrap()),
            );
            (name.to_string(), conds)
        }

        fn line_to_card(line: &str) -> Vec<u64> {
            line.split(',').map(|x| x.parse().unwrap()).collect()
        }

        let (left, rest) = input.split_once("your ticket:").unwrap();
        let rules = left.trim().lines().map(&line_to_rule).collect();
        let (left, rest) = rest.split_once("nearby tickets:").unwrap();
        let my_card = line_to_card(left.trim());
        let tickets = rest.trim().lines().map(&line_to_card).collect();
        Self {
            rules,
            my_ticket: my_card,
            tickets,
        }
    }

    fn get_error_rate(&self) -> u64 {
        let mut failures = Vec::new();
        for ticket in self.tickets.iter() {
            for value in ticket {
                let mut valid = false;
                for rule in self.rules.values() {
                    if valid_by_rule(rule, *value) {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    failures.push(*value)
                }
            }
        }
        failures.iter().sum()
    }

    fn ticket_is_valid(&self, ticket: &[u64]) -> bool {
        ticket
            .iter()
            .map(|x| self.rules.values().any(|rule| valid_by_rule(rule, *x)))
            .all(|x| x)
    }

    fn get_field_order(&self) -> HashMap<String, usize> {
        let nfields = self.rules.len();
        let mut candidates = HashMap::new();
        for rule in self.rules.keys() {
            candidates.insert(rule, vec![true; nfields]);
        }
        for card in self.tickets.iter().filter(|x| self.ticket_is_valid(x)) {
            for (rule_name, rule) in &self.rules {
                for (num, field) in card.iter().enumerate() {
                    if !valid_by_rule(rule, *field) {
                        candidates.get_mut(rule_name).unwrap()[num] = false;
                    }
                }
            }
        }
        /* let mut counts = candidates
            .iter()
            .map(|x| (x.0, x.1.iter().filter(|x| **x).count()))
            .collect::<Vec<_>>();
        counts.sort_by_key(|x| x.1); */
        let mut result = HashMap::new();
        while let Some(candidate) = candidates
            .iter()
            .filter(|x| x.1.iter().filter(|x| **x).count() == 1)
            .map(|x| x.0)
            .next()
        {
            let pos = candidates[*candidate].iter().position(|x| *x).unwrap();
            result.insert((**candidate).clone(), pos);
            for c in &mut candidates {
                c.1[pos] = false
            }
        }
        result
    }
}

fn main() {
    let input = get_input_txt();
    let doc = Document::new(&input);
    println!("Part one: {}", doc.get_error_rate());
    let product: u64 = doc.get_field_order().iter().filter(|x|x.0.starts_with("departure"))
        .map(|x|doc.my_ticket[*x.1]).product();
    println!("Part two: {}", product);

}

#[test]
fn example_input_1() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    let doc = Document::new(input);
    println!("{:?}", doc);
    assert_eq!(71, doc.get_error_rate())
}
