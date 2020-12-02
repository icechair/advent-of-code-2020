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

fn valid_password(line: &String) -> bool {
    let mut parts = line.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 3 {
        return false;
    }
    let minmax = parts[0].split("-").collect::<Vec<&str>>();
    if minmax.len() != 2 {
        return false;
    }
    let min = minmax[0].parse::<i32>().unwrap();
    let max = minmax[1].parse::<i32>().unwrap();
    let search = parts[1].chars().next().unwrap();
    let password = String::from(parts[2]);
    let mut n_search = 0;
    for c in password.chars() {
        if c == search {
            n_search += 1;
        }
    }
    if n_search < min {
        return false;
    }
    if n_search > max {
        return false;
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut valid = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(v) = line {
                if valid_password(&v) {
                    valid += 1;
                }
            }
        }
    }
    println!("{}", valid)
}
