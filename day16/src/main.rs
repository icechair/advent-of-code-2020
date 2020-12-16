use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io;
#[macro_use]
extern crate lazy_static;

fn parse_field_definitions(text: &str) -> HashMap<String, [(usize, usize); 2]> {
    lazy_static! {
        static ref REX: Regex = Regex::new(r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }
    let mut fields = HashMap::new();
    for caps in REX.captures_iter(text) {
        let fieldname = caps.get(1).unwrap().as_str();
        let a = caps[2].parse::<usize>().unwrap();
        let b = caps[3].parse::<usize>().unwrap();
        let c = caps[4].parse::<usize>().unwrap();
        let d = caps[5].parse::<usize>().unwrap();
        fields.insert(String::from(fieldname), [(a, b), (c, d)]);
    }
    return fields;
}

fn part1(text: &str) -> usize {
    let parts = text.split("\n\n").collect::<Vec<&str>>();
    let fields = parse_field_definitions(parts[0]);
    let mut error_rate = 0;
    for (i, ticket) in parts[2].lines().enumerate() {
        if i != 0 {
            let ticket = ticket
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            for value in ticket {
                let mut invalid_value = true;
                for range in fields.values() {
                    if (value >= range[0].0 && value <= range[0].1)
                        || (value >= range[1].0 && value <= range[1].1)
                    {
                        invalid_value = false;
                    }
                }
                if invalid_value {
                    error_rate += value;
                }
            }
        }
    }

    return error_rate;
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("read_to_string failed");
    let error_rate = part1(&text);
    println!("{}", error_rate);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let text = "class: 1-3 or 5-7\n\
            row: 6-11 or 33-44\n\
            seat: 13-40 or 45-50\n\
            \n\
            your ticket:\n\
            7,1,14\n\
            \n\
            nearby tickets:\n\
            7,3,47\n\
            40,4,50\n\
            55,2,20\n\
            38,6,12\n";
        assert_eq!(part1(&text), 71);
    }
}
