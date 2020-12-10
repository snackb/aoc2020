mod validation;

use aoc_input::*;
use std::collections::HashMap;
use validation::*;

type Condition = &'static dyn Fn(&Passport) -> bool;

pub fn field_from_string(input: &str) -> (String, String) {
    let mut split_input = input.split(":");
    let left = split_input.next().unwrap();
    let right = split_input.next().unwrap();
    (left.to_string(), right.to_string())
}

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn from_str(input: &str) -> Self {
        Passport {
            fields: input.split_ascii_whitespace().map(&field_from_string).fold(
                HashMap::new(),
                |mut acc, (name, field)| {
                    acc.insert(name.clone(), field.clone());
                    acc
                },
            ),
        }
    }
}

fn all_except_cid(passport: &Passport) -> bool {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in &required {
        if !passport.fields.keys().any(|x| x == field) {
            return false;
        }
    }
    return true;
}

fn validate_all_except_cid(passport: &Passport) -> bool {
    let required = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in required {
        if !passport.fields.keys().any(|x| x == field) {
            return false;
        }
        if !validate(field, &passport.fields.get(&field.to_string()).unwrap()) {
            return false;
        }
    }
    return true;
}

fn validate(name: &str, value: &str) -> bool {
    match name {
        "byr" => number_digits_between(value, 4, 1920, 2002),
        "iyr" => number_digits_between(value, 4, 2010, 2020),
        "eyr" => number_digits_between(value, 4, 2020, 2030),
        "hgt" => valid_height(value),
        "hcl" => valid_hair_colour(value),
        "ecl" => valid_eye_colour(value),
        "pid" => valid_passport_number(value),
        _ => panic!(format!("should be unreachable {}", name)),
    }
}

fn count_valid_passports(passports: &str, condition: Condition) -> usize {
    passports
        .split("\n\n")
        .map(&Passport::from_str)
        .filter(|x| condition(x))
        .count()
}

fn main() {
    match get_argument_parsed(1) {
        Some(0) => println!(
            "{}",
            count_valid_passports(&get_input_txt(), &all_except_cid)
        ),
        Some(1) => println!(
            "{}",
            count_valid_passports(&get_input_txt(), &validate_all_except_cid)
        ),
        _ => panic!("no!"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_input() {
        let example = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(2, count_valid_passports(example, &all_except_cid))
    }

    #[test]
    fn example_input_2() {
        let example = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(4, count_valid_passports(example, &validate_all_except_cid))
    }
}
