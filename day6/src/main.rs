use aoc_input::*;

type CountingMethod = &'static dyn Fn(&str) -> usize;

fn main() {
    let input = get_input_txt();
    let counting_method: CountingMethod = match get_argument_parsed(1) {
        Some(0) => &unique_chars,
        Some(1) => &all_lines_contain,
        _ => &unique_chars,
    };
    println!("{}", process(&input, counting_method))
}

fn process(input: &str, counting_method: CountingMethod) -> usize {
    input.split("\n\n").map(counting_method).sum()
}

fn unique_chars(input: &str) -> usize {
    let a_z = 'a'..='z'; 
    a_z.filter(|x| input.contains(*x)).count()
}

fn all_lines_contain(input: &str) -> usize {
    let all_lines_contain = |group: &str, ch: char| {
        group.lines().all(|x| x.contains(ch))
    };
    let a_z = 'a'..='z'; 
    a_z.filter(|x| all_lines_contain(input, *x)).count()
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = 
"abc

a
b
c

ab
ac

a
a
a
a

abcx
abcy
abcz

b";
        assert_eq!(17, process(input, &unique_chars));

    }
}
