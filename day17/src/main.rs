use std::collections::HashMap;
use std::env;
use std::io;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3d {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        return Self { x, y, z };
    }
    pub fn neighbours(&self) -> impl Iterator<Item = Self> + '_ {
        let neighbours = (-1..2)
            .map(move |z| {
                return (-1..2)
                    .map(move |y| {
                        return (-1..2).map(move |x| {
                            return *self + Self::new(x, y, z);
                        });
                    })
                    .flatten();
            })
            .flatten()
            .filter(move |p| *p != *self);
        return neighbours;
    }
}

impl ops::Add<Self> for Point3d {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

type CubeMap = HashMap<Point3d, bool>;

fn part1(text: &str) -> usize {
    let mut cubes = CubeMap::new();
    for (row, line) in text.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    cubes.insert(Point3d::new(col as i64, row as i64, 0), true);
                }
                _ => (),
            }
        }
    }

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
    }
    return cubes.len();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point4d {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point4d {
    pub fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        return Self { x, y, z, w };
    }
    pub fn neighbours(&self) -> impl Iterator<Item = Self> + '_ {
        let neighbours = (-1..2)
            .map(move |w| {
                return (-1..2)
                    .map(move |z| {
                        return (-1..2)
                            .map(move |y| {
                                return (-1..2).map(move |x| {
                                    return *self + Self::new(x, y, z, w);
                                });
                            })
                            .flatten();
                    })
                    .flatten();
            })
            .flatten()
            .filter(move |p| *p != *self);
        return neighbours;
    }
}

impl ops::Add<Self> for Point4d {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

type HyperCubeMap = HashMap<Point4d, bool>;

fn part2(text: &str) -> usize {
    let mut cubes = HyperCubeMap::new();
    for (row, line) in text.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    cubes.insert(Point4d::new(col as i64, row as i64, 0, 0), true);
                }
                _ => (),
            }
        }
    }

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
        let mut next = HyperCubeMap::new();
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
    }
    return cubes.len();
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("read_to_string failed");
    if &args[2] == "1" {
        println!("{}", part1(&text));
    }
    if &args[2] == "2" {
        println!("{}", part2(&text));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_part1() {}
}
