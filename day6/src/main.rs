use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn unique_chars(line: &str) -> usize {
    let mut uniques: Vec<char> = Vec::new();
    for c in line.chars() {
        if !uniques.contains(&c) {
            uniques.push(c);
        }
    }
    return uniques.len();
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let mut f = BufReader::new(File::open(&args[1]).expect("File::open failed"));
    let mut buf: Vec<u8> = Vec::new();

    let mut group: String = "".to_owned();
    let mut sum = 0;
    while f.read_until(b'\n', &mut buf).expect("f.read_until failed") != 0 {
        let s = String::from_utf8(buf).expect("String::from_utf8 falied");
        let trimmed = s.trim();
        if trimmed.len() == 0 {
            println!("{}", group);
            sum += unique_chars(&group);
            group.clear();
        } else {
            group.push_str(trimmed);
        }
        buf = s.into_bytes();
        buf.clear();
    }

    if group.len() > 0 {
        sum += unique_chars(&group);
        group.clear();
    }
    println!("{}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {}
}
