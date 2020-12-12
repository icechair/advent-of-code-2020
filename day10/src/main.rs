use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn joltage_difference(x: u64, y: u64) -> (u64, u64, u64) {
    let (mut a, mut b, mut c) = (0, 0, 0);
    match y - x {
        0 => {}
        1 => a += 1,
        2 => b += 1,
        3 => c += 1,
        _ => panic!("too much joltage"),
    }
    return (a, b, c);
}

fn joltage_chain(list: &[u64]) -> (u64, u64, u64) {
    let (mut a, mut b, mut c) = joltage_difference(0, list[0]);

    for i in 0..list.len() - 1 {
        let (x, y, z) = joltage_difference(list[i], list[i + 1]);
        a += x;
        b += y;
        c += z;
    }
    return (a, b, c);
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).expect("File::open failed"));
    let mut list = reader
        .lines()
        .map(|x| {
            x.expect("reader.lines failed")
                .parse::<u64>()
                .expect("parse failed")
        })
        .collect::<Vec<u64>>();

    if &args[2] == "1" {
        list.sort();
        list.push(list[list.len() - 1] + 3);
        let (a, b, c) = joltage_chain(&list);
        println!("{:?}", (a, b, c));
        println!("{:?}", a * c);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_difference() {
        assert_eq!(joltage_difference(0, 1), (1, 0, 0));
        assert_eq!(joltage_difference(0, 2), (0, 1, 0));
        assert_eq!(joltage_difference(0, 3), (0, 0, 1));
        assert_eq!(joltage_difference(5, 6), (1, 0, 0));
        assert_eq!(joltage_difference(5, 7), (0, 1, 0));
        assert_eq!(joltage_difference(5, 8), (0, 0, 1));
    }
    #[test]
    #[should_panic]
    fn test_difference_panic() {
        joltage_difference(0, 4);
    }
    #[test]
    #[should_panic]
    fn test_difference_panic_2() {
        joltage_difference(5, 10);
    }
    #[test]
    fn test_chain() {
        let mut list = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        list.sort();
        list.push(list[list.len() - 1] + 3);
        assert_eq!(joltage_chain(&list), (7, 0, 5));
    }
}
