use std::collections::HashMap;

type Space3 = HashMap<(i64, i64, i64), bool>;
type Space4 = HashMap<(i64, i64, i64, i64), bool>;

fn main() {
    let input = "##...#.#
#..##..#
..#.####
.#..#...
########
######.#
.####..#
.###.#..
";
    let mut space = parse_space3(input);
    cycle_space3_times(&mut space, 6);
    println!("Part one: {}", get_space_count(&space));
    let mut space = parse_space4(input);
    cycle_space4_times(&mut space, 6);
    println!("Part two: {}", get_space4_count(&space));
}

fn parse_space3(input: &str) -> Space3 {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i64, y as i64, 0), char == '#'))
        })
        .collect()
}

fn parse_space4(input: &str) -> Space4 {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i64, y as i64, 0, 0), char == '#'))
        })
        .collect()
}

fn cycle_space3(state: &mut Space3) {
    let mut next_set = HashMap::new();
    for ((x, y, z), value) in state.iter() {
        if *value {
            next_set.insert((x.to_owned(), y.to_owned(), z.to_owned()), *value);
        };
        for (dx, dy, dz) in
            (-1..=1).flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z))))
        {
            let new_coord = (x+dx, y+dy, z+dz);
            if !next_set.contains_key(&new_coord) {
                next_set.insert(new_coord.to_owned(), false);
            }
        }
    }
    for ((x, y, z), value) in next_set.iter_mut() {
        let mut count = 0;
        for (dx, dy, dz) in
            (-1..=1).flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z)))) {
            if dx==0&&dy==0&&dz==0 {continue;}
            if let Some(true) = state.get(&(x+dx, y+dy, z+dz)) {
                count+=1;
            }
        }
        if *value && !(count == 2 || count == 3) {
            *value = false;
        } else if !*value && count == 3{
            *value = true;
        }
    }
    state.clear();
    for (coord, val) in next_set.iter().filter(|(_, val)| **val) {
        state.insert(coord.to_owned(), val.to_owned());
    }
}

fn cycle_space4(state: &mut Space4) {
    let mut next_set = HashMap::new();
    for ((x, y, z, w), value) in state.iter() {
        if *value {
            next_set.insert((x.to_owned(), y.to_owned(), z.to_owned(), w.to_owned()), *value);
        };
        for (dx, dy, dz, dw) in
            (-1..=1).flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).flat_map(move |z| (-1..=1).map(move|w| (x, y, z, w)))))
        {
            let new_coord = (x+dx, y+dy, z+dz, w+dw);
            if !next_set.contains_key(&new_coord) {
                next_set.insert(new_coord.to_owned(), false);
            }
        }
    }
    for ((x, y, z, w), value) in next_set.iter_mut() {
        let mut count = 0;
        for (dx, dy, dz, dw) in
            (-1..=1).flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).flat_map(move |z| (-1..=1).map(move|w| (x, y, z, w))))) {
            if dx==0&&dy==0&&dz==0&&dw==0 {continue;}
            if let Some(true) = state.get(&(x+dx, y+dy, z+dz, w+dw)) {
                count+=1;
            }
        }
        if *value && !(count == 2 || count == 3) {
            *value = false;
        } else if !*value && count == 3{
            *value = true;
        }
    }
    state.clear();
    for (coord, val) in next_set.iter().filter(|(_, val)| **val) {
        state.insert(coord.to_owned(), val.to_owned());
    }
}

fn get_space_count(state: &Space3) -> usize {
    state.iter().filter(|(_, val)|**val).count()
}

fn get_space4_count(state: &Space4) -> usize {
    state.iter().filter(|(_, val)|**val).count()
}

fn cycle_space3_times(state: &mut Space3, n: usize) {
    (0..n).for_each(|_| cycle_space3(state))
}

fn cycle_space4_times(state: &mut Space4, n: usize) {
    (0..n).for_each(|_| cycle_space4(state))
}

#[test]
fn parse_correctly() {
    let input = ".#.
..#
###";
    let space = parse_space3(input);
    assert_eq!(true, *space.get(&(2, 1, 0)).unwrap())
}

#[test]
fn test_cycle() {
    let input = ".#.
..#
###";
    let mut space = parse_space3(input);
    cycle_space3(&mut space);
    println!("{:?}", space);
    assert_eq!(true, *space.get(&(0, 1, -1)).unwrap());
    assert_eq!(true, *space.get(&(1, 3, 0)).unwrap());
}

#[test]
fn test_cycle_6() {
    let input = ".#.
..#
###";
    let mut space = parse_space3(input);
    cycle_space3_times(&mut space, 6);
    assert_eq!(112, get_space_count(&space));
}

#[test]
fn test_cycle4_6() {
    let input = ".#.
..#
###";
    let mut space = parse_space4(input);
    cycle_space4_times(&mut space, 6);
    assert_eq!(848, get_space4_count(&space));
}



