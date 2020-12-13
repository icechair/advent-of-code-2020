use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_bus_notes(text: &str) -> (i64, Vec<&str>) {
    let mut lines = text.lines();
    let arrival = lines
        .next()
        .expect("text arrival line failed")
        .parse::<i64>()
        .expect("text arrival line NaN");
    let bus_row = lines.next().expect("text bus line failed");

    let bus_lines = bus_row.split(",").collect::<Vec<&str>>();

    return (arrival, bus_lines);
}

fn part1(arrival: i64, bus_lines: Vec<&str>) -> i64 {
    let mut fastest = i64::MAX;
    let mut fastest_id = 0;

    for bus in bus_lines {
        if bus == "x" {
            continue;
        }
        let id = bus
            .parse::<i64>()
            .expect(&format!("bus parse failed: {} NaN", bus));
        let mut next_arrival = id;
        while next_arrival < arrival {
            next_arrival += id;
        }
        if next_arrival < fastest {
            fastest = next_arrival;
            fastest_id = id;
        }
    }
    let wait_time = fastest - arrival;

    return fastest_id * wait_time;
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let text = std::fs::read_to_string(&args[1])?;

    let (arrival, bus_lines) = parse_bus_notes(&text);
    println!("{}", part1(arrival, bus_lines));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let text = String::from("939\n7,13,x,x,59,x,31,19");
        let (arrival, bus_lines) = parse_bus_notes(&text);
        assert_eq!(arrival, 939);
        assert_eq!(part1(arrival, bus_lines), 295);
    }
}
