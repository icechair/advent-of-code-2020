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
                let (address, value) = parse_mem(line);
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
                let mut float_addrs = vec![];
                for perm in 0..2u64.pow(n_bits as u32) {
                    let mut addr = float_base;
                    for b in 0..n_bits {
                        match b {
                            b if perm & 1 << b != 0 => {
                                addr = addr | 1 << float_bits[b];
                            }
                            _ => (),
                        }
                    }
                    float_addrs.push(addr);
                }
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
        assert_eq!(bitmask.0, 0b1000000);
        assert_eq!(bitmask.1, !0b10);
        assert_eq!(bitmask.2.len(), 34);

        let bitmask = parse_bitmask("1001XX0X");
        assert_eq!(bitmask.0, 0b10010000);
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
        assert_eq!(program.bitmask.0, 0b1000000);
        assert_eq!(program.bitmask.1, !0b10);
        program.tick_v1("mem[8] = 11");
        program.tick_v1("mem[7] = 101");
        program.tick_v1("mem[8] = 0");
        assert_eq!(program.memory_sum(), 165);
    }
    #[test]
    fn test_program_v2() {
        let mut program = Program::new();

        program.tick_v2("mask = 000000000000000000000000000000X1001X");
        assert_eq!(program.bitmask.0, 0b10010);
        assert_eq!(program.bitmask.1, 0b111111111111111111111111111111001100);
        assert_eq!(
            program.bitmask.2,
            vec![0b010010, 0b010011, 0b110010, 0b110011]
        );

        program.tick_v2("mem[42] = 100");
        program.tick_v2("mask = 00000000000000000000000000000000X0XX");

        assert_eq!(program.bitmask.0, 0);
        assert_eq!(program.bitmask.1, 0b111111111111111111111111111111110100);
        assert_eq!(
            program.bitmask.2,
            vec![0b0000, 0b0001, 0b0010, 0b0011, 0b1000, 0b1001, 0b1010, 0b1011,]
        );

        println!("{:?}", program.bitmask);
        program.tick_v2("mem[26] = 1");
        println!("{:?}", program.memory);
        assert_eq!(program.memory_sum(), 208);
    }
}
