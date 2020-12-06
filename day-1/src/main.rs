use std::{
    fs::File,
    io::{prelude::*, BufReader}
};
use itertools::Itertools;

fn find_doublet(entries: &Vec<i32>) -> (i32, i32) {
    let n = entries.len();
    println!("{:?}", n);
    let mut tail;
    let mut head;
    for i in 0..n {
        tail = entries[n - (i + 1)];
        if tail > 2020 {
            continue;
        }
        for j in 0..n {
            head = entries[j];
            if tail + head > 2020 {
                break;
            }
            if tail + head == 2020 {
                return (tail, head);
            }
        }
    }

    (0, 0)
}

fn find_triplet(entries: &Vec<i32>) -> (i32, i32, i32) {
    let n = entries.len();
    let mut tail;
    let mut head;
    let mut mid;
    for i in 0..n {
        tail = entries[n - (i + 1)];
        if tail > 2020 {
            continue;
        }
        for j in 0..n {
            head = entries[j];
            if tail + head > 2020 {
                break;
            }
            for k in 0..n {
                mid = entries[k];
                if tail + mid + head == 2020 {
                    return (tail, mid, head);
                } else if tail + mid + head > 2020 {
                    break;
                }
            }
        }
    }

    (0, 0, 0)
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let entries: Vec<i32> = buf.lines()
                               .map(|l| l.expect("Failed to read line")
                                         .parse::<i32>()
                                         .expect("Non-number entry found"))
                               .sorted()
                               .collect();


    let (a, b) = find_doublet(&entries);
    println!("{} and {}, sum: {}, product {}", a, b, a+b, a*b);
    let (x, y, z) = find_triplet(&entries);
    println!("{}, {}, and {}, sum: {}, product {}", x, y, z, x+y+z, x*y*z);
}
