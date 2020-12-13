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

fn part2(bus_lines: Vec<&str>) -> i64 {
    let len = bus_lines.len() as i64;
    let mut bus_list = Vec::new();
    let mut gap_list = Vec::new();
    for (i, bus) in bus_lines.into_iter().enumerate() {
        if bus == "x" {
            continue;
        }
        let id = bus
            .parse::<i64>()
            .expect(&format!("bus_parse falied: {} NaN", bus));
        bus_list.push(id);
        gap_list.push(len - (len + i as i64));
    }
    let time: i64 = bus_list.iter().product();
    println!("{:?}", bus_list);
    println!("{:?}", gap_list);
    println!("{:?}", time);
    return 0;
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let text = std::fs::read_to_string(&args[1])?;

    let (arrival, bus_lines) = parse_bus_notes(&text);
    if &args[2] == "1" {
        println!("{}", part1(arrival, bus_lines));
    } else if &args[2] == "2" {
        println!("{}", part2(bus_lines));
    }
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
    #[test]
    fn test_part2() {
        let text = String::from("939\n7,13,x,x,59,x,31,19");
        let (arrival, bus_lines) = parse_bus_notes(&text);
        assert_eq!(arrival, 939);
        assert_eq!(part2(bus_lines), 1068788);

        /*
                let text = String::from("939\n17,x,13,19");
                let (arrival, bus_lines) = parse_bus_notes(&text);
                assert_eq!(arrival, 939);
                assert_eq!(part2(bus_lines), 3417);

                let text = String::from("939\n67,7,59,61");
                let (arrival, bus_lines) = parse_bus_notes(&text);
                assert_eq!(arrival, 939);
                assert_eq!(part2(bus_lines), 754018);

                let text = String::from("939\n67,x,7,59,61");
                let (arrival, bus_lines) = parse_bus_notes(&text);
                assert_eq!(arrival, 939);
                assert_eq!(part2(bus_lines), 779210);

                let text = String::from("939\n67,7,x,59,61");
                let (arrival, bus_lines) = parse_bus_notes(&text);
                assert_eq!(arrival, 939);
                assert_eq!(part2(bus_lines), 1261476);

                let text = String::from("939\n1789,37,47,1889");
                let (arrival, bus_lines) = parse_bus_notes(&text);
                assert_eq!(arrival, 939);
                assert_eq!(part2(bus_lines), 1202161486);

                assert_eq!(true, false);
        */
    }
}
