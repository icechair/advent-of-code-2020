use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn unique_answers(line: &str) -> usize {
    let mut uniques: Vec<char> = Vec::new();
    for c in line.chars() {
        if !uniques.contains(&c) {
            uniques.push(c);
        }
    }
    return uniques.len();
}

fn part1(file: &str) -> usize {
    let mut f = BufReader::new(File::open(file).expect("File::open failed"));
    let mut buf: Vec<u8> = Vec::new();
    let mut group: String = "".to_owned();
    let mut sum = 0;
    while f.read_until(b'\n', &mut buf).expect("f.read_until failed") != 0 {
        let s = String::from_utf8(buf).expect("String::from_utf8 falied");
        let trimmed = s.trim();
        if trimmed.len() == 0 {
            sum += unique_answers(&group);
            group.clear();
        } else {
            group.push_str(trimmed);
        }
        buf = s.into_bytes();
        buf.clear();
    }

    if group.len() > 0 {
        sum += unique_answers(&group);
        group.clear();
    }
    return sum;
}

fn every_answer(group: &str) -> usize {
    let mut answers: Vec<char> = Vec::new();
    let mut members = group.split_whitespace().collect::<Vec<&str>>();
    members.sort_by(|a, b| b.len().cmp(&a.len()));
    for chr in members[0].chars() {
        let mut all_have = true;
        for i in 1..members.len() {
            if !members[i].contains(chr) {
                all_have = false;
            }
        }
        if all_have {
            answers.push(chr);
        }
    }
    return answers.len();
}

fn part2(file: &str) -> usize {
    let mut sum = 0;
    let mut group = String::new();

    let mut f = BufReader::new(File::open(file).expect("File::open failed"));
    let mut buf: Vec<u8> = Vec::new();
    while f.read_until(b'\n', &mut buf).expect("f.read_until failed") != 0 {
        let s = String::from_utf8(buf).expect("String::from_utf8 falied");
        if s == "\n" {
            sum += every_answer(&group);
            group.clear();
        } else {
            group.push_str(&s);
        }
        buf = s.into_bytes();
        buf.clear();
    }
    if group.len() > 0 {
        sum += every_answer(&group);
        group.clear();
    }
    return sum;
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if &args[2] == "1" {
        println!("{}", part1(&args[1]))
    } else if &args[2] == "2" {
        println!("{}", part2(&args[1]))
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {}
}
