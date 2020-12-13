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
        Point(self.0 + o.0, self.1 + o.1)
    }

    pub fn distance(&self, o: Self) -> i64 {
        let dx = o.0 - self.0;
        let dy = o.1 - self.1;
        dx.abs() + dy.abs()
    }
}

struct Ship {
    heading: usize,
    position: Point,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            heading: 1,
            position: Point(0, 0),
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
    pub fn step(&mut self, line: &str) {
        let action = &line[0..1];
        let amount = line[1..]
            .parse::<i64>()
            .expect(&format!("line {} parse failed", &line));
        let modifier = match action {
            "N" => Some(Point(0, -amount)),
            "S" => Some(Point(0, amount)),
            "E" => Some(Point(amount, 0)),
            "W" => Some(Point(-amount, 0)),
            "L" => {
                self.turn(-amount / 90);
                None
            }
            "R" => {
                self.turn(amount / 90);
                None
            }
            "F" => match DIRECTIONS[self.heading] {
                NORTH => Some(Point(0, -amount)),
                SOUTH => Some(Point(0, amount)),
                EAST => Some(Point(amount, 0)),
                WEST => Some(Point(-amount, 0)),
                _ => panic!("Ship.step failed: invalid heading: {}", self.heading),
            },

            _ => panic!(format!("Ship.step failed: line '{}' invalid action", line)),
        };
        if let Some(modifier) = modifier {
            self.position = self.position.add(modifier);
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).expect("File::open failed"));

    if &args[2] == "1" {
        let mut ship = Ship::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                ship.step(&line);
            }
        }
        println!("{}", ship.position.distance(Point(0, 0)))
    } else if &args[2] == "2" {
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
    fn test_ship_step() {
        let mut ship = Ship::new();

        ship.step("F10");
        ship.step("N3");
        ship.step("F7");
        ship.step("R90");
        ship.step("F11");
        assert_eq!(ship.position, Point(17, 8));
        assert_eq!(ship.position.distance(Point(0, 0)), 25);
    }
}
