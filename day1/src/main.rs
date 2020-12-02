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
    let mut records = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                records.push(ip.parse::<i32>().unwrap());
            }
        }
    }

    for (i, x) in records.iter().enumerate() {
        for j in (i + 1)..(records.len()) {
            if x + records[j] == 2020 {
                println!("{}", x * records[j])
            }
        }
    }
}
