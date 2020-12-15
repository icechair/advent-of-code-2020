use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn part1(input: &str, max: usize) -> usize {
    //n, (last_spoken, before_spoken)
    let mut memory = HashMap::<usize, Vec<usize>>::new();
    let input = input
        .split(",")
        .map(|x| {
            x.trim()
                .parse::<usize>()
                .expect(&format!("input parse failed: '{}'", x))
        })
        .collect::<Vec<usize>>();

    let mut spoken = vec![];
    let mut turn = 1;
    let mut n = 0;
    for i in 0..input.len() {
        n = input[i];
        memory.insert(n, vec![turn]);
        spoken.push(n);
        turn += 1;
    }

    while turn <= max {
        if let Some(spoken) = memory.get(&n) {
            if spoken.len() > 1 {
                n = turn - 1 - spoken[spoken.len() - 2];
            } else {
                n = 0;
            }
        } else {
            n = 0;
        }

        if let Some(next) = memory.get_mut(&n) {
            next.push(turn);
        } else {
            memory.insert(n, vec![turn]);
        }

        spoken.push(n);
        turn += 1;
    }
    return n;
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("read_to_string failed");
    let mut max = 2020;
    if &args[2] == "2" {
        max = 30000000;
    }
    let n = part1(&input, max);
    println!("{}", n);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("0,3,6", 2020), 436);

        assert_eq!(part1("1,3,2", 2020), 1);
        assert_eq!(part1("2,1,3", 2020), 10);
        assert_eq!(part1("1,2,3", 2020), 27);
        assert_eq!(part1("2,3,1", 2020), 78);
        assert_eq!(part1("3,2,1", 2020), 438);
        assert_eq!(part1("3,1,2", 2020), 1836);
    }
}
