use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
struct Interval {
    min: usize,
    max: usize,
}

impl Interval {
    fn contains(&self, n: usize) -> bool {
        return self.min <= n && self.max >= n;
    }
}

struct Field {
    name: String,
    intervals: Vec<Interval>,
}

impl Field {
    fn from_line(line: String) -> Self {
        let re = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        let capture = re.captures(&line).unwrap();
        let name = capture[1].to_string();
        let mut intervals = Vec::new();
        intervals.push(Interval{ min: capture[2].parse().unwrap(), max: capture[3].parse().unwrap() });
        intervals.push(Interval{ min: capture[4].parse().unwrap(), max: capture[5].parse().unwrap() });
        return Self { name: name, intervals: intervals };
    }
}

struct Ticket {
    values: Vec<usize>,
}

impl Ticket {
    fn from_line(line: String) -> Self {
        let values: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();
        return Self { values: values };
    }
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let input = BufReader::new(f).lines().map(|l| l.unwrap());

    // parse
    let mut section = 1;
    let mut fields: Vec<Field> = Vec::new();
    let mut tickets: Vec<Ticket> = Vec::new();
    for line in input {
        if line.len() == 0 {
            section += 1;
            continue;
        }
        if  line.contains("ticket") {
            continue;
        }
        match section {
            1 => {
                fields.push(Field::from_line(line));
            },
            2 => {
                println!("Mine {}", line)
            },
            3 => {
                tickets.push(Ticket::from_line(line));
            },
            _ => panic!("!!!"),
        }
    }

    fn check_value(value: usize, fields: &Vec<Field>) -> bool {
        for field in fields  {
            for interval in &field.intervals {
                if interval.contains(value) {
                    return true;
                }
            }
        }

        return false;
    }

    fn count_errors(ticket: &Ticket,  fields:  &Vec<Field>) -> usize {
        let mut invalid = 0;
        for value in &ticket.values {
            if !check_value(*value, &fields) {
                invalid += *value;
            }
        }

        return invalid;
    }

    let mut invalid = 0;
    let mut valid_tickets: Vec<Ticket> = Vec::new();
    for ticket in tickets {
        let errors = count_errors(&ticket, &fields);
        if (errors == 0) {
            valid_tickets.push(ticket);
        }
        invalid += errors;
    }

    println!("Part 1 {}", invalid);
}
