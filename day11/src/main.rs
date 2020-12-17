use aoc_input::*;
use std::string::ToString;

#[derive(Debug, Copy, Clone)]
enum Space {
    Seat(bool, bool),
    Floor,
}

enum SpaceStatus {
    Seat(bool),
    Empty,
    Boundary,
}

impl SpaceStatus {
    fn get_seat_value(&self) -> Option<bool> {
        match self {
            Self::Seat(status) => Some(*status),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Floor {
    spaces: Vec<Vec<Space>>,
    read_left: bool,
    part: u64,
}

impl Floor {
    fn new_from_pt1(input: &str) -> Self {
        let lines = input.lines();
        Self {
            spaces: lines
                .map(|x| {
                    x.chars()
                        .map(|x| match x {
                            'L' => Space::Seat(false, false),
                            '.' => Space::Floor,
                            _ => panic!("yikes"),
                        })
                        .collect()
                })
                .collect(),
            read_left: false,
            part: 1,
        }
    }

    fn new_from_pt2(input: &str) -> Self {
        let lines = input.lines();
        Self {
            spaces: lines
                .map(|x| {
                    x.chars()
                        .map(|x| match x {
                            'L' => Space::Seat(false, false),
                            '.' => Space::Floor,
                            _ => panic!("yikes"),
                        })
                        .collect()
                })
                .collect(),
            read_left: false,
            part: 2,
        }
    }

    fn get_space_status(&self, row: i64, col: i64) -> SpaceStatus {
        if row.is_negative() || col.is_negative() {
            return SpaceStatus::Boundary;
        }
        let (r, c) = (row as usize, col as usize);
        let space = self.spaces.get(r).and_then(|x| x.get(c));
        match space {
            None => SpaceStatus::Boundary,
            Some(Space::Floor) => SpaceStatus::Empty,
            Some(Space::Seat(left, right)) => {
                SpaceStatus::Seat(self.read_left && *left || !self.read_left && *right)
            }
        }
    }

    fn get_next_state(&self, row: usize, col: usize) -> bool {
        match self.part {
            1 => self.get_next_state_1(row, col),
            2 => self.get_next_state_2(row, col),
            _ => panic!(),
        }
    }

    fn get_next_state_1(&self, row: usize, col: usize) -> bool {
        let mut count = 0;
        for r_delta in -1..=1 {
            for c_delta in -1..=1 {
                if r_delta == 0 && c_delta == 0 {
                    continue;
                }
                if let SpaceStatus::Seat(status) =
                    self.get_space_status(row as i64 + r_delta, col as i64 + c_delta)
                {
                    if status {
                        count += 1
                    }
                }
            }
        }
        match count {
            0 => true,
            1..=3 => self
                .get_space_status(row as i64, col as i64)
                .get_seat_value()
                .unwrap(),
            _ => false,
        }
    }

    fn get_next_state_2(&self, row: usize, col: usize) -> bool {
        let mut count = 0;
        for r_delta in -1..=1 {
            for c_delta in -1..=1 {
                if r_delta == 0 && c_delta == 0 {
                    continue;
                }
                if self.occupied_seat_in_direction(row as i64, col as i64, (r_delta, c_delta)) {
                    count += 1
                }
            }
        }
        match count {
            0 => true,
            1..=4 => self
                .get_space_status(row as i64, col as i64)
                .get_seat_value()
                .unwrap(),
            _ => false,
        }
    }

    fn occupied_seat_in_direction(&self, row: i64, col: i64, dir: (i64, i64)) -> bool {
        let mut magnitude = 1;
        loop {
            let (rd, cd) = (dir.0 * magnitude, dir.1 * magnitude);
            match self.get_space_status(row + rd, col + cd) {
                SpaceStatus::Seat(false) | SpaceStatus::Boundary => return false,
                SpaceStatus::Seat(true) => return true, 
                SpaceStatus::Empty => {magnitude += 1; continue}
            };
        }
    }

    fn update(&mut self) -> bool {
        let mut updated_any = false;
        for row in 0..self.spaces.len() {
            for col in 0..self.spaces[row].len() {
                self.spaces[row][col] = match self.spaces[row][col] {
                    Space::Seat(left, right) => {
                        if self.read_left {
                            Space::Seat(left, self.get_next_state(row, col))
                        } else {
                            Space::Seat(self.get_next_state(row, col), right)
                        }
                    }
                    other => other,
                };
                if let Space::Seat(left, right) = self.spaces[row][col] {
                    updated_any = left != right || updated_any;
                };
            }
        }
        self.read_left = !self.read_left;
        updated_any
    }

    fn get_occupied_count(&self) -> usize {
        let mut count = 0;
        for row in &self.spaces {
            for space in row {
                match space {
                    Space::Seat(left, right) => {
                        if *left && self.read_left || *right && !self.read_left {
                            count += 1
                        }
                    }
                    Space::Floor => {}
                };
            }
        }
        count
    }
}

fn main() {
    let input = get_input_txt();
    let mut floor = Floor::new_from_pt1(&input);
    while floor.update() {}
    println!("Part 1: {}", floor.get_occupied_count());
    let mut floor = Floor::new_from_pt2(&input);
    while floor.update() {}
    println!("Part 2: {}", floor.get_occupied_count())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_input_1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let mut floor = Floor::new_from_pt1(input);
        while floor.update() {}
        assert_eq!(37, floor.get_occupied_count());
    }

    #[test]
    fn example_input_2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let mut floor = Floor::new_from_pt2(input);
        while floor.update() {}
        assert_eq!(26, floor.get_occupied_count());
    }
}
