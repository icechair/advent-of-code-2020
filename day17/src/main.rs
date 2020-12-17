use std::collections::HashMap;
use std::env;
use std::io;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        return Self { x, y, z };
    }
    pub fn neighbours(&self) -> impl Iterator<Item = Point> + '_ {
        let neighbours = (-1..2)
            .map(move |z| {
                return (-1..2)
                    .map(move |y| {
                        return (-1..2).map(move |x| {
                            return *self + Point::new(x, y, z);
                        });
                    })
                    .flatten();
            })
            .flatten()
            .filter(move |p| *p != *self);
        return neighbours;
    }
}

impl ops::Add<Self> for Point {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<Self> for Point {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

type CubeMap = HashMap<Point, bool>;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("read_to_string failed");

    let mut cubes = CubeMap::new();
    for (row, line) in text.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    cubes.insert(Point::new(col as i64, row as i64, 0), true);
                }
                _ => (),
            }
        }
    }

    //part1
    for _ in 0..6 {
        let mut next = cubes.clone();
        //expand the dimension
        for (cube, _) in &cubes {
            for n in cube.neighbours() {
                if let None = next.get(&n) {
                    next.insert(n, false);
                }
            }
        }
        cubes = next;
        let mut next = CubeMap::new();
        //simulate the step
        for (cube, state) in &cubes {
            let n_active = cube
                .neighbours()
                .filter(|n| {
                    if let Some(state) = cubes.get(n) {
                        return *state;
                    }
                    return false;
                })
                .count();
            if *state {
                if n_active == 2 || n_active == 3 {
                    next.insert(*cube, true);
                }
            } else {
                if n_active == 3 {
                    next.insert(*cube, true);
                }
            }
        }
        cubes = next;
        //println!("{:?}", cubes);
    }
    println!("{}", cubes.len());

    Ok(())
}
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_part1() {}
}
