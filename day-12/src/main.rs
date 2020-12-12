use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use regex::Regex;
use std::f32::consts::PI;

struct Ship {
    x: f32,
    y: f32,
    heading: f32,
}

impl Ship {
    fn go(&mut self, heading: f32, amount: f32) -> () {
        self.x = self.x + amount * heading.cos();
        self.y = self.y + amount * heading.sin();
    }

    fn rotate(&mut self, amount: f32) -> () {
        self.heading = (self.heading + amount) % (2.0 * PI);
    }
}

struct Waypoint {
    x: f32,
    y: f32,
}

impl Waypoint {
    fn go(&mut self, heading: f32, amount: f32) -> () {
        self.x = self.x + amount * heading.cos();
        self.y = self.y + amount * heading.sin();
    }

    fn rotate(&mut self, amount: f32) -> () {
        let x = self.x * amount.cos() - self.y * amount.sin();
        let y = self.y * amount.cos() + self.x * amount.sin();
        self.x  = x;
        self.y = y;
    }
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let input = BufReader::new(f).lines().map(|l| l.unwrap());
    let instr_re = Regex::new(r"^([RLFNSEW])(\d+)").unwrap();

    let mut ship = Ship { x: 0.0, y: 0.0, heading: 0.0 };
    let mut waypoint = Waypoint {  x: 10.0, y: 1.0 };

    for line in input {
        let cap = instr_re.captures(&line).unwrap();
        let direction = &cap[1];
        let amount: f32 = cap[2].parse::<i32>().unwrap() as f32;

        // PART 1
        //match direction {
        //    "N" => ship.go(PI / 2.0, amount),
        //    "S" => ship.go(3.0 * PI / 2.0, amount),
        //    "E" => ship.go(0.0, amount),
        //    "W" => ship.go(PI, amount),
        //    "R" => ship.rotate(amount.to_radians() * -1.0),
        //    "L" => ship.rotate(amount.to_radians()),
        //    "F" => ship.go(ship.heading, amount),
        //    _ => panic!("{} is not a valid direction", direction),
        //}

        // PART 2
        match direction {
            "N" => waypoint.go(PI / 2.0, amount),
            "S" => waypoint.go(3.0 * PI / 2.0, amount),
            "E" => waypoint.go(0.0, amount),
            "W" => waypoint.go(PI, amount),
            "R" => waypoint.rotate(2.0 * PI - amount.to_radians()),
            "L" => waypoint.rotate(amount.to_radians()),
            "F" => ship.go((waypoint.y).atan2(waypoint.x),
                           amount * (waypoint.y.powi(2) + waypoint.x.powi(2)).sqrt()),
            _ => panic!("{} is not a valid direction", direction),
        }
    }

    println!("Ship is at ( {}, {} ), result {}", 
                ship.x.round() as i32,
                ship.y.round() as i32,
                (ship.x.abs() + ship.y.abs()).round() as i32);
}
