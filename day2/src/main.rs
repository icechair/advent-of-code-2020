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

fn valid_password_part1(line: &String) -> bool {
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
fn valid_password_part2(line: &String) -> bool {
    let mut parts = line.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 3 {
        return false;
    }
    let minmax = parts[0].split("-").collect::<Vec<&str>>();
    if minmax.len() != 2 {
        return false;
    }
    let idx = minmax[0].parse::<usize>().unwrap() - 1;
    let idy = minmax[1].parse::<usize>().unwrap() - 1;
    let search = parts[1].chars().next().unwrap();
    let password = String::from(parts[2]).chars().collect::<Vec<char>>();

    if (password[idx] == search && password[idy] == search) {
        return false;
    }
    if (password[idx] == search || password[idy] == search) {
        return true;
    }

    false
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut valid = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(v) = line {
                if args[1] == "1" {
                    if valid_password_part1(&v) {
                        valid += 1;
                    }
                } else if args[1] == "2" {
                    if valid_password_part2(&v) {
                        valid += 1;
                    }
                }
            }
        }
    }
    println!("{}", valid)
}
