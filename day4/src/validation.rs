pub fn number_digits_between(value: &str, digits: usize, lower: usize, upper: usize) -> bool {
    if value.len() != digits {
        return false;
    };
    if let Ok(number) = value.parse::<usize>() {
        number >= lower && number <= upper
    } else {
        false
    }
}

pub fn valid_height(value: &str) -> bool {
    if value.ends_with("cm") {
        match value.trim_end_matches("cm").parse::<usize>() {
            Ok(num) => num >= 150 && num <= 193,
            _ => false,
        }
    } else if value.ends_with("in") {
        match value.trim_end_matches("in").parse::<usize>() {
            Ok(num) => num >= 59 && num <= 76,
            _ => false,
        }
    } else {false}
}

pub fn valid_hair_colour(value: &str) -> bool {
    value.starts_with("#") &&
    value.trim_start_matches("#").len() == 6 &&
    value.trim_start_matches("#").chars().all(
        |x| ('0' <= x && x <= '9') || ('a' <= x && x <= 'f')
    )
}

pub fn valid_eye_colour(value: &str) -> bool { 
    (&["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]).contains(&value)
}

pub fn valid_passport_number(value: &str) -> bool {
    value.len() == 9 &&
    value.chars().all(|x|x.is_numeric())
}