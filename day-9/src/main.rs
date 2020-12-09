use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

fn sum_exists(entries: &Vec<u64>, target: u64) -> bool{
    for e1 in entries {
        for e2 in entries {
            if e1 == e2 {
                continue;
            }
            if e1 + e2 == target {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let input: Vec<u64> = buf.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let mut invalid_number: u64 = 0;

    for (n, target) in input[25..].iter().enumerate() {
        let candidates: Vec<u64> = input[n..n + 25].to_vec();
        if !sum_exists(&candidates, *target) {
            invalid_number = *target;
        }
    }

    println!("Part 1: {}", invalid_number);

    for (n, value) in input.iter().enumerate() {
        let mut sum: u64 = *value;
        let mut values: Vec<u64> = [*value].to_vec();
        for vv in input[n + 1..].iter() {
            sum = sum + vv;
            values.push(*vv);
            if sum == invalid_number {
                values.sort();
                println!("Part 2: {}", values.first().unwrap() + values.last().unwrap());

            } else if sum > invalid_number {
                break;
            }
        }
    }

}
