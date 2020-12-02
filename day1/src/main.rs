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

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut records = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                records.push(ip.parse::<i32>().unwrap());
            }
        }
    }
    if args[1] == "1" {
        for (i, x) in records.iter().enumerate() {
            for j in (i + 1)..(records.len()) {
                if x + records[j] == 2020 {
                    println!("{}", x * records[j])
                }
            }
        }
    } else if args[1] == "2" {
        for (i, _) in records.iter().enumerate() {
            for j in (i + 1)..(records.len()) {
                for k in (j + 1)..(records.len()) {
                    if records[i] + records[j] + records[k] == 2020 {
                        println!("{}", records[i] * records[j] * records[k])
                    }
                }
            }
        }
    }
}
