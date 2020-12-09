use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).expect("File::open failed"));
    let list = reader
        .lines()
        .map(|x| {
            x.expect("reader.lines failed")
                .parse::<i64>()
                .expect("parse failed")
        })
        .collect::<Vec<i64>>();

    if &args[2] == "1" {
        for i in 25..list.len() {
            let mut preamble = &list[i - 25..i];
            if !find_sum(preamble, list[i]) {
                println!("{}", list[i]);
                return Ok(());
            }
        }
    } else if &args[2] == "2" {
    }

    Ok(())
}

fn find_sum(list: &[i64], n: i64) -> bool {
    for i in 0..list.len() - 1 {
        for j in i + 1..list.len() {
            if list[i] + list[j] == n {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stuff() {
        let list = vec![1, 2, 3];
        assert_eq!(find_sum(&list, 3), true);
        assert_eq!(find_sum(&list, 5), true);
        assert_eq!(find_sum(&list, 4), true);
        assert_eq!(find_sum(&list, 6), false);
    }
}
