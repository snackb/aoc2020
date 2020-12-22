use aoc_input::*;

enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Ship {
    x: i64,
    y: i64,
    heading: i64,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            heading: 90,
        }
    }

    fn change_heading(&mut self, change: i64, direction: Direction) {
        self.heading = match direction {
            Direction::Left => (self.heading - change).rem_euclid(360),
            Direction::Right => (self.heading + change).rem_euclid(360),
        }
    }

    fn forward(&mut self, distance: i64) {
        match self.heading {
            0 => self.y += distance,
            90 => self.x += distance,
            180 => self.y -= distance,
            270 => self.x -= distance,
            _ => panic!("Wrong borough."),
        }
    }

    fn apply_input(&mut self, instr: &str) {
        let num = instr[1..].parse().unwrap();
        match instr.chars().next().unwrap() {
            'N' => self.y += num,
            'S' => self.y -= num,
            'E' => self.x += num,
            'W' => self.x -= num,
            'L' => self.change_heading(num, Direction::Left),
            'R' => self.change_heading(num, Direction::Right),
            'F' => self.forward(num),
            _ => panic!("I don't know what to do!"),
        }
    }

    fn get_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Waypoint {
    x: i64,
    y: i64,
    wx: i64,
    wy: i64,
}

impl Waypoint {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }

    fn change_heading(&mut self, change: i64, direction: Direction) {
        let (mut nwx, mut nwy) = (self.wx, self.wy);
        match (change, direction) {
            (0, _) | (360, _) => (),
            (90, Direction::Left) | (270, Direction::Right) => {
                nwx = -self.wy;
                nwy = self.wx
            }
            (180, _) => {
                nwx = -self.wx;
                nwy = -self.wy
            }
            (270, Direction::Left) | (90, Direction::Right) => {
                nwx = self.wy;
                nwy = -self.wx
            }
            _ => panic!("This is not the Bronx."),
        };
        self.wx = nwx;
        self.wy = nwy;
    }

    fn forward(&mut self, distance: i64) {
        self.x += self.wx * distance;
        self.y += self.wy * distance;
    }

    fn apply_input(&mut self, instr: &str) {
        let num = instr[1..].parse().unwrap();
        match instr.chars().next().unwrap() {
            'N' => self.wy += num,
            'S' => self.wy -= num,
            'E' => self.wx += num,
            'W' => self.wx -= num,
            'L' => self.change_heading(num, Direction::Left),
            'R' => self.change_heading(num, Direction::Right),
            'F' => self.forward(num),
            _ => panic!("I don't know what to do!"),
        }
    }

    fn get_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let input = get_input_txt();
    let mut ship = Ship::new();
    for line in input.lines() {
        ship.apply_input(line);
    }
    println!("Part one: {}", ship.get_distance());
    let input = get_input_txt();
    let mut waypoint = Waypoint::new();
    for line in input.lines() {
        waypoint.apply_input(line);
    }
    println!("Part two: {}", waypoint.get_distance());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heading() {
        let mut ship = Ship {
            x: 0,
            y: 0,
            heading: 0,
        };
        ship.change_heading(90, Direction::Right);
        assert_eq!(90, ship.heading);
        ship.change_heading(360, Direction::Right);
        assert_eq!(90, ship.heading);
        ship.change_heading(360, Direction::Left);
        assert_eq!(90, ship.heading);
        ship.change_heading(90, Direction::Left);
        assert_eq!(0, ship.heading);
        ship.change_heading(270, Direction::Left);
        assert_eq!(90, ship.heading);
    }

    #[test]
    fn example_input_1() {
        let input = "F10
N3
F7
R90
F11";
        let mut ship = Ship::new();
        for line in input.lines() {
            ship.apply_input(line);
        }
        assert_eq!(25, ship.get_distance())
    }

    #[test]
    fn example_input_2() {
        let input = "F10
N3
F7
R90
F11";
        let mut waypoint = Waypoint::new();
        for line in input.lines() {
            waypoint.apply_input(line);
            println!("{}", line);
            println!("{:?}", waypoint);
        }
        assert_eq!(286, waypoint.get_distance())
    }
}
