use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const MAX_ROWS: usize = 128;
const MAX_COLS: usize = 8;

fn seat_id(row: usize, column: usize) -> usize {
    return row * 8 + column;
}

fn binary_search(line: &str, max: usize) -> usize {
    let mut left = 0;
    let mut right = max - 1;
    for c in line.chars() {
        let center = (left + right) / 2;
        match c {
            'F' | 'L' => {
                right = center;
            }
            'B' | 'R' => {
                left = center + 1;
            }
            _ => eprintln!("error row part \"{}\", '{}' is invalid", line, c),
        };
    }
    return left;
}

fn parse_pass(line: &str) -> usize {
    let row_part = &line[0..7];
    let col_part = &line[7..];
    let row = binary_search(row_part, MAX_ROWS);
    let col = binary_search(col_part, MAX_COLS);
    return seat_id(row, col);
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut max_id = 0;
    if let Ok(lines) = read_lines(&args[1]) {
        for (i, line) in lines.enumerate() {
            match line {
                Ok(row) => {
                    let id = parse_pass(&row);
                    if id > max_id {
                        max_id = id;
                    }
                }
                Err(e) => {
                    eprintln!("error in line {}: {}", i, e);
                    return Err(e);
                }
            }
        }
    }
    println!("{}", max_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pass() {
        assert_eq!(parse_pass("FBFBBFFRLR"), 357);
        assert_eq!(parse_pass("BFFFBBFRRR"), 567);
        assert_eq!(parse_pass("FFFBBBFRRR"), 119);
        assert_eq!(parse_pass("BBFFBBFRLL"), 820);
    }
}
