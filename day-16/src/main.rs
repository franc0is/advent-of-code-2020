use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader}
};

use regex::Regex;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Hash)]
struct Interval {
    min: usize,
    max: usize,
}

impl Interval {
    fn contains(&self, n: usize) -> bool {
        return self.min <= n && self.max >= n;
    }
}

#[derive(Debug, Clone, Hash)]
struct Field {
    name: String,
    intervals: Vec<Interval>,
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Field {}

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
    let mut my_ticket: Ticket = Ticket { values: Vec::new() };
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
                my_ticket = Ticket::from_line(line);
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

    let num_fields = fields.len();
    let mut possibilities: HashMap<usize, Vec<Field>> = HashMap::new();

    fn is_matching_field(field: &Field, values: Vec<usize>) -> bool {
        for v in values {
            if !field.intervals[0].contains(v) && !field.intervals[1].contains(v) {
                return false
            }
        }

        return true;
    }

    for field in fields {
        for i in 0..num_fields {
            let values: Vec<usize> = valid_tickets.iter().map(|t| t.values[i]).collect();
            if is_matching_field(&field, values) {
                let mut v: Vec<Field> = match possibilities.get(&i) {
                    None => Vec::new(),
                    Some(x) => x.to_vec(),
                };
                v.push(field.clone());
                possibilities.insert(i, v);
            }
        }
    }

    let mut mappings: HashMap<usize, Field> = HashMap::new();
    for i in 0..num_fields {
        for i in 0..num_fields {
            let p = match possibilities.get(&i) {
                None => continue,
                Some(x) => x.clone(),
            };
            let mut field: Field = Field {name: "".to_string(), intervals: Vec::new()};
    
            if p.len() == 1 {
                field = p.get(0).unwrap().clone();
                possibilities.remove(&i);
                mappings.insert(i, field.clone());
            } else {
                continue;
            }
    
            for j in 0..num_fields {
                let v = match possibilities.get_mut(&j) {
                    None => continue,
                    Some(x) => x,
                };
                let index = v.iter().position(|f| *f.name == field.name);
                if index.is_some() {
                    v.remove(index.unwrap());
                }
            }
            for p in possibilities.values() {
            }
        }
    }

    for (i, v) in my_ticket.values.iter().enumerate() {
        println!("{} {}", mappings.get(&i).unwrap().name, v);
    }
}
