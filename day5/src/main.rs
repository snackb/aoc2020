use aoc_input::*;

#[derive(Debug)]
struct Seat(usize, usize);

impl Seat {
    fn from_str(input: &str) -> Self {
        if input.len() != 10 {
            panic!(format!("{}", input.len()))
        }
        let row_chars = input.chars().take(7);
        let col_chars = input.chars().skip(7).take(3);
        let row = chars_to_bin(row_chars.collect(), 'B');
        let col = chars_to_bin(col_chars.collect(), 'R');
        return Self(row, col);
    }

    fn seat_number(&self) -> usize {
        let Self(row, col) = self;
        return (row * 8) + col;
    }
}

fn chars_to_bin(chars: Vec<char>, one: char) -> usize {
    chars.into_iter().rev().enumerate().fold(
        0,
        |acc, (i, val)| if val == one { acc + (1 << i) } else { acc },
    )
}

fn main() {
    let input = get_input_txt();
    let seats = input.lines().map(&Seat::from_str).map(|x| x.seat_number());
    let mut seats = seats.collect::<Vec<usize>>();
    seats.sort();
    for i in 1..seats.len() - 2 {
        if seats[i - 1] != seats[i] - 1 || seats[i + 1] != seats[i] + 1 {
            println!("Found: {}", seats[i])
        } else {
            //println!("{} {} {}", seats[i-1], seats[i], seats[i+1])
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "BBFFBBFRLL";
        let seat = Seat::from_str(input);
        println!("{:?}", seat);
        assert_eq!(820, Seat::from_str(input).seat_number());
    }
}
