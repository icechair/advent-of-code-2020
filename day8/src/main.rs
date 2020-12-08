use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Program {
    pc: i32,
    acc: i32,
}

impl Program {
    pub fn new() -> Program {
        return Program { pc: 0, acc: 0 };
    }

    fn nop(&mut self) {
        self.pc += 1;
    }

    fn acc(&mut self, arg: i32) {
        self.acc += arg;
        self.pc += 1;
    }

    fn jmp(&mut self, arg: i32) {
        self.pc += arg;
    }

    fn execute_opcode(&mut self, line: &str) {
        let mut parts = line.split(" ");
        if let Some(opcode) = parts.next() {
            if let Some(arg) = parts.next() {
                if let Ok(n) = arg.parse::<i32>() {
                    match opcode {
                        "nop" => self.nop(),
                        "acc" => self.acc(n),
                        "jmp" => self.jmp(n),
                        _ => panic!("execute_opcode: opcode '{}' not implemented"),
                    }
                    return;
                }
            }
        }
        panic!("execute_opcode: invalid line '{}'", line);
    }

    pub fn step(&mut self, rom: &Vec<String>) -> usize {
        if self.pc >= 0 {
            self.execute_opcode(&rom[self.pc as usize]);
        } else {
            panic!("program.step: pc is negative");
        }
        return self.pc as usize;
    }

    pub fn get_acc(&self) -> i32 {
        return self.acc;
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut program = Program::new();

    let reader = BufReader::new(File::open(&args[1]).expect("File::open failed"));
    let mut rom = reader
        .lines()
        .map(|row| row.expect("reader.lines failed"))
        .collect::<Vec<String>>();

    if &args[2] == "1" {
        let mut history = Vec::<usize>::new();
        loop {
            let pc = program.step(&rom);
            if history.contains(&pc) {
                eprintln!("infinite loop at instruction: {}, {}", pc, rom[pc]);
                break;
            }
            history.push(pc);
        }
        println!("{}", program.get_acc());
    } else if &args[2] == "2" {
        let mut cursor = 0;
        loop {
            for i in cursor..rom.len() {
                if rom[i].starts_with("nop") || rom[i].starts_with("jmp") {
                    cursor = i;
                    swap_nop_jmp(&mut rom, cursor);
                    break;
                }
            }
            let mut history = Vec::<usize>::new();
            loop {
                let pc = program.step(&rom);
                if history.contains(&pc) {
                    eprintln!("infinite loop at instruction: {}, {}", pc, rom[pc]);
                    break;
                }
                history.push(pc);
                if pc >= rom.len() {
                    println!("{}", program.get_acc());
                    return Ok(());
                }
            }
            swap_nop_jmp(&mut rom, cursor);
            cursor += 1;
            program.reset();
        }
    }

    Ok(())
}

fn swap_nop_jmp(rom: &mut Vec<String>, i: usize) {
    let tmp = String::from(&rom[i]);
    if tmp.starts_with("nop") {
        rom[i] = String::from("jmp");
    } else if tmp.starts_with("jmp") {
        rom[i] = String::from("nop");
    }
    rom[i].push_str(&tmp[3..]);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_stuff() {
        assert_eq!("+13".parse::<i32>().unwrap(), 13);
        assert_eq!("-13".parse::<i32>().unwrap(), -13);
    }
}
