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

fn check_slope(left: usize, down: usize, input: &String) -> usize {
    let mut n_trees = 0;
    let mut width = 0;
    let mut x = left;
    let mut y = 0;

    if let Ok(lines) = read_lines(&input) {
        for line in lines {
            if let Ok(row) = line {
                if width == 0 {
                    width = row.len()
                }
                if y < down {
                    y += 1;
                    continue;
                }
                y = 1;
                let chars = row.chars().collect::<Vec<char>>();
                if chars[x] == '#' {
                    n_trees += 1;
                }
                x += left;
                if x >= width {
                    x -= width;
                }
            }
        }
    }
    n_trees
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut n_trees = 1;

    if &args[2] == "1" {
        n_trees = check_slope(3, 1, &args[1]);
    } else if &args[2] == "2" {
        n_trees *= check_slope(1, 1, &args[1]);
        n_trees *= check_slope(3, 1, &args[1]);
        n_trees *= check_slope(5, 1, &args[1]);
        n_trees *= check_slope(7, 1, &args[1]);
        n_trees *= check_slope(1, 2, &args[1]);
    }
    println!("{}", n_trees)
}
