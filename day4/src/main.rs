use std::collections::HashSet;
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

fn valid_year(value: &str, min: usize, max: usize) -> bool {
    if value.len() != 4 {
        return false;
    }
    return match value.parse::<usize>() {
        Ok(year) => {
            if year >= min && year <= max {
                return true;
            }
            false
        }
        Err(_) => false,
    };
}

fn valid_height(value: &str) -> bool {
    if value.len() == 0 {
        return false;
    }
    let height = match value[0..value.len() - 2].parse::<usize>() {
        Ok(v) => v,
        Err(_) => 0,
    };
    let unit = &value[value.len() - 2..];
    return match unit {
        "cm" => height >= 150 && height <= 193,
        "in" => height >= 59 && height <= 76,
        _ => false,
    };
}

fn valid_hair_color(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }
    if !value.starts_with("#") {
        return false;
    }
    for c in value[1..].chars() {
        if !"0123456789abcdef".contains(c) {
            return false;
        }
    }
    return true;
}

fn valid_eye_color(value: &str) -> bool {
    return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value);
}

fn valid_passport_id(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }
    for c in value.chars() {
        if !"0123456789".contains(c) {
            return false;
        }
    }
    return true;
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        if self.byr == "" {
            false
        } else if self.iyr == "" {
            false
        } else if self.eyr == "" {
            false
        } else if self.hgt == "" {
            false
        } else if self.hcl == "" {
            false
        } else if self.ecl == "" {
            false
        } else if self.pid == "" {
            false
        } else {
            true
        }
    }

    fn is_valid_part2(&self) -> bool {
        valid_year(&self.byr, 1920, 2002)
            && valid_year(&self.iyr, 2010, 2020)
            && valid_year(&self.eyr, 2020, 2030)
            && valid_height(&self.hgt)
            && valid_hair_color(&self.hcl)
            && valid_eye_color(&self.ecl)
            && valid_passport_id(&self.pid)
    }
}

fn parse_passport(input: &str) -> Passport {
    let mut passport = Passport {
        byr: "".to_string(),
        iyr: "".to_string(),
        eyr: "".to_string(),
        hgt: "".to_string(),
        hcl: "".to_string(),
        ecl: "".to_string(),
        pid: "".to_string(),
        cid: "".to_string(),
    };
    let parts = input.trim().split(" ");
    for part in parts {
        let kv = part.split(":").collect::<Vec<&str>>();
        match kv[0] {
            "byr" => passport.byr = kv[1].to_string(),
            "iyr" => passport.iyr = kv[1].to_string(),
            "eyr" => passport.eyr = kv[1].to_string(),
            "hgt" => passport.hgt = kv[1].to_string(),
            "hcl" => passport.hcl = kv[1].to_string(),
            "ecl" => passport.ecl = kv[1].to_string(),
            "pid" => passport.pid = kv[1].to_string(),
            "cid" => passport.cid = kv[1].to_string(),
            _ => {}
        }
    }
    passport
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut passports = HashSet::new();
    let mut passport_line = String::new();

    if let Ok(lines) = read_lines(&args[1]) {
        for (i, line) in lines.enumerate() {
            match line {
                Ok(row) => {
                    if row.len() > 1 {
                        passport_line = format!("{} {}", passport_line, row);
                    } else {
                        let passport = parse_passport(&passport_line);
                        passports.insert(passport);
                        passport_line = String::new();
                    }
                }
                Err(e) => {
                    eprintln!("error in line {}: {}", i, e);
                    return Err(e);
                }
            }
        }
        if passport_line.len() > 0 {
            let passport = parse_passport(&passport_line);
            passports.insert(passport);
        }
    }
    let mut n_valid = 0;
    if &args[2] == "1" {
        for p in passports {
            if p.is_valid_part1() {
                n_valid += 1;
            }
        }
    } else if &args[2] == "2" {
        for p in passports {
            if p.is_valid_part2() {
                n_valid += 1;
            }
        }
    }
    println!("{}", n_valid);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_year() {
        assert_eq!(valid_year("2020", 2010, 2020), true);
        assert_eq!(valid_year("2020", 2010, 2019), false);
        assert_eq!(valid_year("2000", 2001, 2019), false);
        assert_eq!(valid_year("", 2001, 2019), false);
        assert_eq!(valid_year("22222", 2001, 2019), false);
    }

    #[test]
    fn test_valid_height() {
        assert_eq!(valid_height(""), false);
        assert_eq!(valid_height("60in"), true);
        assert_eq!(valid_height("190cm"), true);
        assert_eq!(valid_height("190in"), false);
        assert_eq!(valid_height("190"), false);
    }

    #[test]
    fn test_valid_color() {
        assert_eq!(valid_hair_color("#123abc"), true);
        assert_eq!(valid_hair_color("#123abz"), false);
        assert_eq!(valid_hair_color("123abc"), false);

        assert_eq!(valid_eye_color("brn"), true);
        assert_eq!(valid_eye_color("wat"), false);
    }

    #[test]
    fn test_valid_passport_id() {
        assert_eq!(valid_passport_id("000000001"), true);
        assert_eq!(valid_passport_id("01234567890"), false);
    }
}
