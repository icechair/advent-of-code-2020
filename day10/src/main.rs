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
    let (mut a, mut b, mut c) = (0, 0, 0);
    for window in list.windows(2) {
        let (x, y, z) = joltage_difference(window[0], window[1]);
        a += x;
        b += y;
        c += z;
    }
    return (a, b, c);
}

fn max_joltage_arragements(list: &[u64]) -> u64 {
    let mut slices = Vec::new();
    let mut current = Vec::new();
    for window in list.windows(2) {
        match window[1] - window[0] {
            1 => current.push(window[0]),
            3 => {
                current.push(window[0]);
                slices.push(current);
                current = Vec::new();
            }
            _ => {}
        }
    }
    return slices
        .iter()
        .map(|slice| match slice.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!("unexpected slice size"),
        })
        .product();
}

fn create_adapter_list(list: &[u64]) -> Vec<u64> {
    let mut list = Vec::from(list);
    list.sort();
    list.insert(0, 0);
    list.push(list.last().unwrap() + 3);
    return Vec::from(list);
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).expect("File::open failed"));
    let list = reader
        .lines()
        .map(|x| {
            x.expect("reader.lines failed")
                .parse::<u64>()
                .expect("parse failed")
        })
        .collect::<Vec<u64>>();

    let list = create_adapter_list(&list);
    if &args[2] == "1" {
        let (a, _, c) = joltage_chain(&list);
        println!("{:?}", a * c);
    } else if &args[2] == "2" {
        println!("{}", (max_joltage_arragements(&list)));
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
        let list = create_adapter_list(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);
        assert_eq!(joltage_chain(&list), (7, 0, 5));

        assert_eq!(max_joltage_arragements(&list), 8);
        let list = create_adapter_list(&[
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]);
        assert_eq!(joltage_chain(&list), (22, 0, 10));

        assert_eq!(max_joltage_arragements(&list), 19208);
    }
}
