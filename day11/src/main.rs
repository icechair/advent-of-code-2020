use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Floor,
    Chair,
    Person,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Floor => write!(f, "."),
            Cell::Chair => write!(f, "L"),
            Cell::Person => write!(f, "#"),
        }
    }
}

struct SeatLayout {
    grid: Vec<Cell>,
    width: usize,
}

impl SeatLayout {
    pub fn new(lines: Vec<String>) -> SeatLayout {
        let iter = lines.iter();
        let seats = SeatLayout {
            width: lines[0].len(),
            grid: iter.fold(Vec::<Cell>::new(), |mut acc, c| {
                let mut clist = c
                    .chars()
                    .into_iter()
                    .map(|c| match c {
                        '.' => Cell::Floor,
                        'L' => Cell::Chair,
                        '#' => Cell::Person,
                        _ => panic!("invalid char in grid"),
                    })
                    .collect::<Vec<Cell>>();
                acc.append(&mut clist);
                acc
            }),
        };

        return seats;
    }

    fn index(&self, row: i64, col: i64) -> i64 {
        row * (self.width as i64) + col
    }

    fn col(&self, index: i64) -> i64 {
        index % self.width as i64
    }

    fn row(&self, index: i64) -> i64 {
        index / self.width as i64
    }

    fn get_cell_at(&self, row: i64, col: i64) -> Cell {
        if row < 0 {
            return Cell::Floor;
        }
        if col < 0 {
            return Cell::Floor;
        }
        if col >= self.width as i64 {
            return Cell::Floor;
        }
        let index = self.index(row, col);
        if index >= 0 && index < self.grid.len() as i64 {
            return self.grid[index as usize];
        }
        return Cell::Floor;
    }

    fn kernel(&self, index: usize) -> [Cell; 8] {
        let row = self.row(index as i64);
        let col = self.col(index as i64);
        return [
            self.get_cell_at(row - 1, col - 1),
            self.get_cell_at(row - 1, col),
            self.get_cell_at(row - 1, col + 1),
            self.get_cell_at(row, col - 1),
            self.get_cell_at(row, col + 1),
            self.get_cell_at(row + 1, col - 1),
            self.get_cell_at(row + 1, col),
            self.get_cell_at(row + 1, col + 1),
        ];
    }

    pub fn step(&mut self) -> usize {
        let mut next_grid = self.grid.clone();
        for (i, cell) in self.grid.iter().enumerate() {
            if cell == &Cell::Floor {
                continue;
            }
            let kernel = self.kernel(i);
            let mut n_occupied = 0;
            for k in &kernel {
                match k {
                    Cell::Person => n_occupied += 1,
                    _ => (),
                };
            }
            next_grid[i] = match cell {
                Cell::Floor => Cell::Floor,
                Cell::Chair => match n_occupied {
                    0 => Cell::Person,
                    _ => Cell::Chair,
                },
                Cell::Person => match n_occupied {
                    n if n >= 4 => Cell::Chair,
                    _ => Cell::Person,
                },
            }
        }
        self.grid = next_grid;
        return self.get_occupied();
    }

    pub fn get_occupied(&self) -> usize {
        let occupied = self
            .grid
            .iter()
            .map(|c| match c {
                Cell::Person => 1,
                _ => 0,
            })
            .sum();
        return occupied;
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).expect("File::open failed"));
    let list = reader
        .lines()
        .map(|x| x.expect("zeug"))
        .collect::<Vec<String>>();

    let mut seats = SeatLayout::new(list);
    let mut prev = seats.get_occupied();
    let mut n = 0;
    //println!("{}", seats);
    while n < 1000 {
        let current = seats.step();
        //println!("{}", seats);
        if current == prev {
            break;
        }
        prev = current;
        n += 1;
    }
    println!("{}", prev);
    Ok(())
}

impl fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "---\n")?;
        for (i, cell) in self.grid.iter().enumerate() {
            if i > 0 && i % self.width == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", cell)?;
        }
        write!(f, "\n---")
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_seats() {}
}
