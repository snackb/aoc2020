use aoc_input::*;

fn main() {
    let input = get_input_txt();
    let x_step = get_argument_parsed(1).unwrap();
    let y_step = get_argument_parsed(2).unwrap();
    println!("{}", count_trees(&input, x_step, y_step))
}

fn count_trees(input: &str, x_step: usize, y_step: usize) -> usize {
    let hill: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let (mut x, mut y) = (0, 0);
    let width = hill[0].len();
    let height = hill.len();
    let mut sum = 0;
    loop {
        if y >= height - 1 {
            return sum;
        }
        x = x + x_step;
        y = y + y_step;
        if y > height - 1 {
            continue;
        }
        let adjusted_x = x % width;
        sum += match hill[y][adjusted_x] {
            '#' => 1,
            '.' => 0,
            _ => panic!("Unknown char."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        assert_eq!(7, count_trees(input, 3, 1));
    }
}
