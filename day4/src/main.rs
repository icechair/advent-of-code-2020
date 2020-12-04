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
    fn is_valid(&self) -> bool {
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
                        println!("{}", passport_line);
                        //println!("{:?}", passport);
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
            println!("{}", passport_line);
            //println!("{:?}", passport);
            passports.insert(passport);
        }
    }
    let mut n_valid = 0;
    for p in passports {
        if p.is_valid() {
            n_valid += 1;
        }
    }
    println!("{}", n_valid);

    Ok(())
}
