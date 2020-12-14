use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
#[macro_use]
extern crate lazy_static;

fn parse_mem(line: &str) -> (u64, u64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }
    let cap = RE
        .captures_iter(line)
        .next()
        .expect(&format!("parse_mem failed: not a memory line: '{}'", line));

    let address = cap[1].parse::<u64>().expect("address NaN");
    let value = cap[2].parse::<u64>().expect("value NaN");
    return (address, value);
}
fn parse_bitmask(line: &str) -> (u64, u64, Vec<u64>) {
    let mut set_bitmask = 0;
    let mut unset_bitmask = u64::MAX;
    let mut float_bits = vec![];
    for (i, c) in line.chars().rev().enumerate() {
        match c {
            '1' => set_bitmask |= 1 << i,
            '0' => unset_bitmask &= !(1 << i),
            'X' => float_bits.push(i as u64),
            _ => (),
        }
    }
    return (set_bitmask, unset_bitmask, float_bits);
}

struct Program {
    memory: HashMap<u64, u64>,
    bitmask: (u64, u64, Vec<u64>),
}

impl Program {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
            bitmask: (0, u64::MAX, vec![]),
        }
    }
    pub fn tick_v1(&mut self, line: &str) {
        match line {
            s if s.starts_with("mask") => self.bitmask = parse_bitmask(&line[7..]),
            s if s.starts_with("mem") => {
                let (address, mut value) = parse_mem(line);
                let (set_mask, unset_mask, _) = self.bitmask;
                self.memory.insert(address, value | set_mask & unset_mask);
            }
            _ => panic!("Program.tick: invalid line: '{}'", line),
        }
    }

    pub fn tick_v2(&mut self, line: &str) {
        match line {
            s if s.starts_with("mask") => {
                let (float_base, mut whitelist, float_bits) = parse_bitmask(&line[7..]);
                whitelist = !whitelist;
                let n_bits = float_bits.len();
                let float_addrs = (0..2usize.pow(n_bits as u32))
                    .map(|perm| {
                        (0..n_bits).fold(float_base, |addr, x| match x {
                            x if perm & 1 << x != 0 => addr | 1 << float_bits[x],
                            _ => addr,
                        })
                    })
                    .collect();
                self.bitmask = (float_base, whitelist, float_addrs);
            }
            s if s.starts_with("mem") => {
                let (address, value) = parse_mem(line);
                let (_, whitelist, float_addrs) = &self.bitmask;
                for float_addr in float_addrs {
                    self.memory.insert(address & whitelist | float_addr, value);
                }
            }
            _ => panic!("Program.tick: invalid line: '{}'", line),
        }
    }

    pub fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).expect("File::open failed"));

    for i in 0..2usize.pow(2) {
        println!("{:?}", i);
    }
    let mut program = Program::new();
    if &args[2] == "1" {
        for line in reader.lines() {
            program.tick_v1(&line.expect("reader.line failed"));
        }
    } else if &args[2] == "2" {
        for line in reader.lines() {
            program.tick_v2(&line.expect("reader.line failed"));
        }
    }
    println!("{}", program.memory_sum());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bitmask() {
        let bitmask = parse_bitmask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(bitmask.0, (1 << 6));
        assert_eq!(bitmask.1, (!(1 << 1)));
        assert_eq!(bitmask.2.len(), 36 - 2);

        let bitmask = parse_bitmask("1001XX0X");
        assert_eq!(bitmask.0, (1 << 7) | (1 << 4));
        assert_eq!(bitmask.1, !0b1100010);
        assert_eq!(bitmask.2.len(), 3);
    }

    #[test]
    fn test_parse_mem() {
        assert_eq!(parse_mem("mem[24196] = 465592"), (24196, 465592));
        assert_eq!(parse_mem("mem[17683] = 909049"), (17683, 909049));
    }

    #[test]
    fn test_program_v1() {
        let mut program = Program::new();

        program.tick_v1("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        program.tick_v1("mem[8] = 11");
        program.tick_v1("mem[7] = 101");
        program.tick_v1("mem[8] = 0");
        assert_eq!(program.memory_sum(), 165);
    }
    #[test]
    fn test_program_v2() {
        let mut program = Program::new();

        program.tick_v2("mask = 000000000000000000000000000000X1001X");
        println!("{:?}", program.bitmask);
        program.tick_v2("mem[42] = 100");
        println!("{:?}", program.memory);
        program.tick_v2("mask = 00000000000000000000000000000000X0XX");
        println!("{:?}", program.bitmask);
        program.tick_v2("mem[26] = 1");
        println!("{:?}", program.memory);
        assert_eq!(program.memory_sum(), 208);
    }
}
