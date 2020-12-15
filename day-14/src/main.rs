use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let input = BufReader::new(f).lines().map(|l| l.unwrap());

    let mask_re = Regex::new(r"^mask = ([01X]+)$").unwrap();
    let mem_re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut mask: u64 = 0;
    let mut pattern: u64 = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();

//    for line in input {
//        if mask_re.is_match(&line) {
//            let input_str = &mask_re.captures(&line).unwrap()[1];
//            pattern = input_str.chars().fold(0, |acc, c| {
//                (acc << 1) | match c {
//                    '0' | 'X' => 0,
//                    '1' => 1,
//                    _ => panic!("Invalid character {}", c),
//                }
//            });
//            mask = input_str.chars().fold(0, |acc, c| {
//                (acc << 1) | match c {
//                    '1' | 'X' => 1,
//                    '0' => 0,
//                    _ => panic!("Invalid character {}", c),
//                }
//            });
//            //println!("input: {} pattern: {:x}, mask: {:x}", line, pattern, mask);
//        } else {
//            let cap = mem_re.captures(&line).unwrap();
//            let addr: u64 = cap[1].parse().unwrap();
//            let mut value: u64 = cap[2].parse().unwrap();
//            value = (value & mask) | pattern;
//            memory.insert(addr, value);
//        }
//    }
//
//    println!("Part 1: {}", memory.values().fold(0, |acc, v| acc + v));
//
//    mask = 0;
//    pattern = 0;
//    memory = HashMap::new();
    let mut fixed_bits: u64 = 0;
    let mut floating_bits: Vec<u64> = Vec::new();

    for line in input {
        if mask_re.is_match(&line) {
            //println!("new mask {}", &line);
            let input_str = &mask_re.captures(&line).unwrap()[1];
            // fixed bits
            fixed_bits = input_str.chars().fold(0, |acc, c| {
                acc << 1 | match c {
                    '1' => 1,
                    _ => 0,
                }
            });
            // floating bits
            let a: Vec<(usize, char)> = input_str.chars().enumerate().collect();
            //println!("a {:?}", &a);
            floating_bits = a.iter().filter(|(i,c)| *c == 'X').map(|(i, c)| {
                //println!("{}", i);
                1 << (35 - i)
            }).collect();
            //println!("floating bits: {:?}", floating_bits);
        } else {
            let cap = mem_re.captures(&line).unwrap();
            let addr: u64 = cap[1].parse().unwrap();
            let mut value: u64 = cap[2].parse().unwrap();
            let mut addresses: HashSet<u64> = HashSet::new();
            addresses.insert(addr | fixed_bits);
            for bit in &floating_bits {
                let mut  ins: Vec<u64> = Vec::new();
                for a in &addresses {
                    ins.push(a | bit);
                    ins.push(a & !bit);
                }
                for a in ins {
                    //println!("Inserting {}", a);
                    addresses.insert(a);
                }
            }
            for a in &addresses {
                //println!("Address: {:b} ({})", a, a);
                memory.insert(*a, value);
            }
        }
    }

    println!("Part 2: {}", memory.values().fold(0, |acc, v| acc + v));
}
