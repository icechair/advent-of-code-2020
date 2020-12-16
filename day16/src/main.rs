use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io;
#[macro_use]
extern crate lazy_static;

type FieldDef = HashMap<String, [(usize, usize); 2]>;
type Ticket = Vec<usize>;

fn parse_field_definitions(text: &str) -> FieldDef {
    lazy_static! {
        static ref REX: Regex = Regex::new(r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }
    let mut fields = HashMap::new();
    for caps in REX.captures_iter(text) {
        let fieldname = caps.get(1).unwrap().as_str();
        let a = caps[2].parse::<usize>().unwrap();
        let b = caps[3].parse::<usize>().unwrap();
        let c = caps[4].parse::<usize>().unwrap();
        let d = caps[5].parse::<usize>().unwrap();
        fields.insert(String::from(fieldname), [(a, b), (c, d)]);
    }
    return fields;
}

fn parse_notes(text: String) -> (FieldDef, Ticket, Vec<Ticket>) {
    let parts = text.split("\n\n").collect::<Vec<&str>>();
    let fields = parse_field_definitions(parts[0]);
    let ticket = parts[1]
        .lines()
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let tickets = parts[2]
        .lines()
        .skip(1)
        .map(|x| x.split(",").map(|x| x.parse::<usize>().unwrap()).collect())
        .collect();

    return (fields, ticket, tickets);
}

fn part1(fields: &FieldDef, tickets: Vec<Ticket>) -> (usize, Vec<Ticket>) {
    let mut valid_tickets = Vec::new();
    let mut error_rate = 0;
    for ticket in &tickets {
        let mut invalid_ticket = false;
        for value in ticket {
            let mut invalid_value = true;
            for range in fields.values() {
                if (value >= &range[0].0 && value <= &range[0].1)
                    || (value >= &range[1].0 && value <= &range[1].1)
                {
                    invalid_value = false;
                }
            }
            if invalid_value {
                error_rate += value;
                invalid_ticket = true;
            }
        }
        if !invalid_ticket {
            valid_tickets.push(ticket.clone());
        }
    }

    return (error_rate, valid_tickets);
}

fn get_ticket_cols(tickets: Vec<Ticket>) -> Vec<Ticket> {
    let mut ticket_cols = vec![];
    for ticket in tickets {
        for (i, value) in ticket.iter().enumerate() {
            if let None = ticket_cols.get(i) {
                ticket_cols.push(vec![]);
            }
            ticket_cols[i].push(*value);
        }
    }
    return ticket_cols;
}

fn get_column_possibilities(
    fields: &FieldDef,
    ticket_cols: Vec<Ticket>,
) -> Vec<Vec<(usize, String)>> {
    let mut columns = vec![];
    for (i, col) in ticket_cols.iter().enumerate() {
        columns.push(vec![]);
        for (fieldname, ranges) in fields {
            let mut in_field_range = true;
            for value in col {
                if value < &ranges[0].0
                    || value > &ranges[0].1 && value < &ranges[1].0
                    || value > &ranges[1].1
                {
                    in_field_range = false;
                    break;
                }
            }
            if in_field_range {
                columns[i].push((i, fieldname.clone()));
            }
        }
    }
    return columns;
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("read_to_string failed");
    let (fields, ticket, tickets) = parse_notes(text);

    let (error_rate, valid_tickets) = part1(&fields, tickets);
    // valid_tickets.push(ticket);
    let ticket_cols = get_ticket_cols(valid_tickets);
    let mut possibilities = get_column_possibilities(&fields, ticket_cols);
    println!("{:?}", error_rate);

    let mut columns = HashMap::<usize, String>::new();

    while columns.len() < fields.len() {
        possibilities.sort_by(|a, b| a.len().cmp(&b.len()));
        let next = possibilities.remove(0);
        if next.len() == 1 {
            let (id, label) = &next[0];
            columns.insert(*id, label.clone());

            for list in possibilities.iter_mut() {
                list.retain(|(_, x)| label != x);
            }
        }
    }

    let mut product = 1;
    for (id, field) in columns {
        if field.starts_with("departure") {
            product *= ticket[id];
        }
    }
    println!("{}", product);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let text = String::from(
            "class: 1-3 or 5-7\n\
            row: 6-11 or 33-44\n\
            seat: 13-40 or 45-50\n\
            \n\
            your ticket:\n\
            7,1,14\n\
            \n\
            nearby tickets:\n\
            7,3,47\n\
            40,4,50\n\
            55,2,20\n\
            38,6,12\n",
        );
        let (fields, ticket, tickets) = parse_notes(text);
        assert_eq!(ticket, Ticket::from([7, 1, 14]));
        assert_eq!(
            tickets,
            vec![
                Ticket::from([7, 3, 47]),
                Ticket::from([40, 4, 50]),
                Ticket::from([55, 2, 20]),
                Ticket::from([38, 6, 12])
            ]
        );
        let (error_rate, valid_tickets) = part1(&fields, tickets);
        assert_eq!(error_rate, 71);
        assert_eq!(valid_tickets.len(), 1);
    }

    #[test]
    fn test_part2() {
        let text = String::from(
            "class: 0-1 or 4-19\n\
            row: 0-5 or 8-19\n\
            seat: 0-13 or 16-19\n\
            \n\
            your ticket:\n\
            11,12,13\n\
            \n\
            nearby tickets:\n\
            3,9,18\n\
            15,1,5\n\
            5,14,9",
        );
        let (fields, ticket, tickets) = parse_notes(text);

        let (error_rate, valid_tickets) = part1(&fields, tickets);
        let ticket_cols = get_ticket_cols(valid_tickets);
        let columns = get_column_possibilities(&fields, ticket_cols);
        println!("{:?}", columns);
        assert_eq!(true, false);
    }
}
