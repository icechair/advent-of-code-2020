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

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut n_trees = 0;
    let mut width = 0;
    let mut x = 0;
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(row) = line {
                if width == 0 {
                    width = row.len();
                } else {
                    let chars = row.chars().collect::<Vec<char>>();
                    if chars[x] == '#' {
                        n_trees += 1;
                    }
                }

                x += 3;
                if x >= width {
                    x -= width
                }
            }
        }
    }
    println!("{}", n_trees)
}
