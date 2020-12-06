use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use itertools::Itertools;

const NUM_ROWS: u32 = 128;
const SEATS_IN_ROW: u32 = 8;

fn bp2seat(bp: &String) -> u32 {
    let row_chars = &bp[..7];
    let seat_chars = &bp[7..];

    let mut row = NUM_ROWS as f32 / 2.0;
    let mut interval = row;
    //println!("r {}", row);
    for c in row_chars.chars() {
        interval = interval / 2.0;
        match c {
            'F' => {
                row = row - interval;
                //println!("r {}", row);
            },
            'B' => {
                row = row + interval;
                //println!("r {}", row);
            },
            _ => println!("error: {}", c)
        }
    }

    let mut seat = SEATS_IN_ROW as f32 / 2.0;
    let mut interval = seat;
    //println!("s {}", seat);
    for c in seat_chars.chars() {
        interval = interval / 2.0;
        match c {
            'L' => {
                seat = seat - interval;
                //println!("s {}", seat);
            },
            'R' => {
                seat = seat + interval;
                //println!("s {}", seat);
            },
            _ => println!("error: {}", c)
        }
    }

    //println!("Row: {}, Seat: {}", row as u32, seat as u32);

    let seat_id = row as u32 * 8 + seat as u32;

    return seat_id;
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let boarding_passes = buf.lines().map(|l| l.unwrap());
    let seat_ids = boarding_passes.map(|bp| bp2seat(&bp)).sorted();
    let mut previous = 0;
    for sid in seat_ids {
        if previous != 0 && sid - previous > 1 {
            println!("{} is missing", sid - 1);
        }
        previous = sid;
    }
    //println!("{:?}", seat_ids);
//    let mut max_seat_id = 0;
//    for bp in boarding_passes {
//        let seat_id = bp2seat(&bp);
//        if seat_id > max_seat_id {
//            max_seat_id = seat_id;
//        }
//    }
//
//    println!("Max seat id: {}", max_seat_id);
}
