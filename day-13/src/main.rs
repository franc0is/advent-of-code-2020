use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use num_integer;

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let mut input = BufReader::new(f).lines().map(|l| l.unwrap());

    let ready_time:usize = input.next().unwrap().parse().unwrap();

    #[derive(Debug)]
    struct Bus {
        index: usize,
        time: usize,
    }

    let line = input.next().unwrap();
    let all_entries = line.split(',');
    let mut buses: Vec<Bus> = Vec::new();
    for (index, entry) in all_entries.enumerate() {
        if entry != "x" {
            buses.push(Bus { index: index, time: entry.parse().unwrap() });
        }
    }

    //println!("Buses: {:?}", buses);

    let mut best_bus = None;
    let mut best_wait: usize = 0;

    for bus in &buses {
        let wait_time = bus.time - (ready_time % bus.time);
        if best_bus.is_none() || wait_time < best_wait {
            best_bus = Some(bus);
            best_wait = wait_time;
        }
    }

    println!("part 1 {}", best_bus.unwrap().time * best_wait);

    let mut t = 0;
    let mut stride = buses[0].time;
    let mut idx = 1;
    loop {
        let bus = &buses[idx];
        t += stride;
        if t % bus.time == bus.time - bus.index % bus.time {
            stride = num_integer::lcm(bus.time, stride);
            idx += 1;
        }
        if idx > buses.len() - 1 {
            break;
        }
    }

    println!("part 2 {}", t);
}
