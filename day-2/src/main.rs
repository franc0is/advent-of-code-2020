use std::{
    fs::File,
    io::{prelude::*, BufReader}
};
use regex::Regex;

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let re = Regex::new(r"(\d+)-(\d+) (\D): (\D+)").unwrap();
    let mut policy_one_valid = 0;
    let mut policy_two_valid = 0;
    for line in buf.lines() {
        let l = line.unwrap();
        let m = re.captures(&l).expect("RE does not match");
        let i1: usize = m[1].parse().unwrap();
        let i2: usize = m[2].parse().unwrap();
        let character: String = m[3].to_string();
        let password: String = m[4].to_string();

        // Policy one
        let occurences = password.matches(&character).count();
        if i1 <= occurences && i2 >= occurences {
            policy_one_valid = policy_one_valid + 1;
        }

        // Policy two
        let c = character.chars().nth(0).unwrap();
        let check_i1 = match password.chars().nth(i1 - 1) {
            Some(letter) => (letter == c),
            None => continue
        };
        let check_i2 = match password.chars().nth(i2 - 1) {
            Some(letter) => (letter == c),
            None => continue
        };
        if check_i1 != check_i2 {
            policy_two_valid = policy_two_valid + 1;
        }
    }
    println!("{} passwords are valid per policy one", policy_one_valid);
    println!("{} passwords are valid per policy two", policy_two_valid);
}
