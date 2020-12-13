use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug)]
struct Point(i64, i64);

const NORTH: Point = Point(0, -1);
const EAST: Point = Point(1, 0);
const SOUTH: Point = Point(0, 1);
const WEST: Point = Point(-1, 0);
const DIRECTIONS: [Point; 4] = [NORTH, EAST, SOUTH, WEST];

impl Point {
    pub fn add(&self, o: Self) -> Self {
        Self(self.0 + o.0, self.1 + o.1)
    }

    pub fn mul(&self, scalar: i64) -> Self {
        Self(self.0 * scalar, self.1 * scalar)
    }

    pub fn rotate_right(&self, amount: i64) -> Self {
        let mut p = self.clone();
        for _ in 0..amount {
            p = Self(-p.1, p.0);
        }
        return p;
    }

    pub fn rotate_left(&self, amount: i64) -> Self {
        let mut p = self.clone();
        for _ in 0..amount {
            p = Self(p.1, -p.0);
        }

        return p;
    }

    pub fn distance(&self, o: Self) -> i64 {
        let dx = o.0 - self.0;
        let dy = o.1 - self.1;
        dx.abs() + dy.abs()
    }
}

fn parse_action(line: &str) -> (&str, i64) {
    let action = &line[0..1];
    let amount = line[1..]
        .parse::<i64>()
        .expect(&format!("line {} parse failed", &line));

    (action, amount)
}

struct Ship {
    heading: usize,
    position: Point,
    waypoint: Point,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            heading: 1,
            position: Point(0, 0),
            waypoint: Point(10, -1),
        }
    }

    fn turn(&mut self, amount: i64) {
        let mut heading = self.heading as i64;
        heading = (heading + amount) % DIRECTIONS.len() as i64;
        if heading < 0 {
            heading += DIRECTIONS.len() as i64;
        }

        self.heading = heading as usize;
    }

    pub fn part1(&mut self, line: &str) {
        let (action, amount) = parse_action(line);
        match action {
            "N" => self.position = self.position.add(Point(0, -amount)),
            "S" => self.position = self.position.add(Point(0, amount)),
            "E" => self.position = self.position.add(Point(amount, 0)),
            "W" => self.position = self.position.add(Point(-amount, 0)),
            "L" => self.turn(-amount / 90),
            "R" => self.turn(amount / 90),
            "F" => match DIRECTIONS[self.heading] {
                NORTH => self.position = self.position.add(Point(0, -amount)),
                SOUTH => self.position = self.position.add(Point(0, amount)),
                EAST => self.position = self.position.add(Point(amount, 0)),
                WEST => self.position = self.position.add(Point(-amount, 0)),
                _ => panic!("Ship.part1 failed: invalid heading: {}", self.heading),
            },

            _ => panic!("Ship.part1 failed: line '{}' invalid action", line),
        }
    }

    fn part2(&mut self, line: &str) {
        let (action, amount) = parse_action(line);
        match action {
            "N" => self.waypoint = self.waypoint.add(Point(0, -amount)),
            "S" => self.waypoint = self.waypoint.add(Point(0, amount)),
            "E" => self.waypoint = self.waypoint.add(Point(amount, 0)),
            "W" => self.waypoint = self.waypoint.add(Point(-amount, 0)),
            "L" => self.waypoint = self.waypoint.rotate_left(amount / 90),
            "R" => self.waypoint = self.waypoint.rotate_right(amount / 90),
            "F" => self.position = self.position.add(self.waypoint.mul(amount)),
            _ => panic!("Ship.part2 failed: invalid action: {}", line),
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).expect("File::open failed"));

    let mut ship = Ship::new();
    if &args[2] == "1" {
        for line in reader.lines() {
            if let Ok(line) = line {
                ship.part1(&line);
            }
        }
        println!("{}", ship.position.distance(Point(0, 0)))
    } else if &args[2] == "2" {
        for line in reader.lines() {
            if let Ok(line) = line {
                ship.part2(&line);
            }
        }
        println!("{}", ship.position.distance(Point(0, 0)))
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_turn() {
        let mut ship = Ship::new();
        assert_eq!(DIRECTIONS[ship.heading], EAST);
        ship.turn(1);
        assert_eq!(DIRECTIONS[ship.heading], SOUTH);
        ship.turn(-2);
        assert_eq!(DIRECTIONS[ship.heading], NORTH);
        ship.turn(-4);
        assert_eq!(DIRECTIONS[ship.heading], NORTH);
        ship.turn(-1);
        assert_eq!(DIRECTIONS[ship.heading], WEST);
        ship.turn(-1);
        assert_eq!(DIRECTIONS[ship.heading], SOUTH);
    }

    #[test]
    fn test_ship_part1() {
        let mut ship = Ship::new();

        ship.part1("F10");
        assert_eq!(ship.position, Point(10, 0));
        ship.part1("N3");
        assert_eq!(ship.position, Point(10, -3));
        ship.part1("F7");
        assert_eq!(ship.position, Point(17, -3));
        ship.part1("R90");
        assert_eq!(DIRECTIONS[ship.heading], SOUTH);
        ship.part1("F11");
        assert_eq!(ship.position, Point(17, 8));
        assert_eq!(ship.position.distance(Point(0, 0)), 25);
    }
    #[test]
    fn test_ship_part2() {
        let mut ship = Ship::new();
        assert_eq!(ship.waypoint, Point(10, -1));
        assert_eq!(ship.position, Point(0, 0));

        ship.part2("F10");
        assert_eq!(ship.waypoint, Point(10, -1));
        assert_eq!(ship.position, Point(100, -10));

        ship.part2("N3");
        assert_eq!(ship.waypoint, Point(10, -4));
        assert_eq!(ship.position, Point(100, -10));

        ship.part2("F7");
        assert_eq!(ship.waypoint, Point(10, -4));
        assert_eq!(ship.position, Point(170, -38));

        ship.part2("R90");
        assert_eq!(ship.waypoint, Point(4, 10));
        assert_eq!(ship.position, Point(170, -38));

        ship.part2("F11");
        assert_eq!(ship.waypoint, Point(4, 10));
        assert_eq!(ship.position, Point(214, 72));
    }
}
