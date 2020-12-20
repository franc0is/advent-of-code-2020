use std::{
    ops::Add,
    fs::File,
    io::{prelude::*, BufReader}
};

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum SeatState {
    FLOOR,
    OCCUPIED,
    EMPTY,
}

impl SeatState {
    fn from_char(c: &char) -> SeatState {
        match  c {
            'L' => SeatState::EMPTY,
            '.' => SeatState::FLOOR,
            '#' => SeatState::OCCUPIED,
            _ => panic!("Invalid seat input {}", c)
        }
    }
}

impl std::fmt::Display for SeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SeatState::EMPTY => write!(f, "L"),
            SeatState::FLOOR => write!(f, "."),
            SeatState::OCCUPIED => write!(f, "#")
        }
    }
}

struct SeatMap {
    width: i32,
    height: i32,
    seats: Vec<SeatState>,
}

impl SeatMap {
    const ADJACENT_MAP: [Point;8] = [
        Point { x: -1, y: -1 },
        Point { x:  0, y: -1 },
        Point { x:  1, y: -1 },
        Point { x: -1, y:  0 },
        Point { x:  1, y:  0 },
        Point { x: -1, y:  1 },
        Point { x:  0, y:  1 },
        Point { x:  1, y:  1 },
    ];

    fn get(&self, p: Point) -> Option<SeatState> {
        if p.x < 0 || p.x >= self.width ||
           p.y < 0 || p.y >= self.height {
            // out of bounds
            return None;
        }

        let i = (p.y * self.width + p.x) as usize;
        Some(self.seats[i])
    }

    fn get_adjacent(&self, p: Point) -> Vec<SeatState> {
        return Self::ADJACENT_MAP.iter()
                                 .map(|adjacent| self.get(*adjacent + p))
                                 .filter(|opt| opt.is_some())
                                 .map(|opt| opt.unwrap())
                                 .collect();
    }

    fn count_adjacent(&self, p: Point) -> usize {
        self.get_adjacent(p).iter().fold(0, |acc, adjacent| {
            match adjacent {
                SeatState::EMPTY | SeatState::FLOOR => {
                    acc
                },
                SeatState::OCCUPIED => {
                    acc + 1
                }
            }
        })
    }

    fn count_occupied(&self) -> usize {
        self.seats.iter().fold(0, |acc, s| {
            if *s == SeatState::OCCUPIED {
                acc + 1
            } else {
                acc
            }
        })
    }
}


impl std::fmt::Display for SeatMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, seat) in self.seats.iter().enumerate() {
            if i != 0 && i % (self.width as usize) == 0 {
                write!(f, "\n");
            }
            write!(f, "{}", seat);
        }
        Ok(())
    }
}


//fn count_adjacent_part2(seatmap: &SeatMap, i: usize, j: usize) -> usize {
//    let h = seatmap.len();
//    let w = seatmap[0].len();
//    let mut count = 0;
//
//    // look right
//    for k in j..w {
//        if (i == 2 && j == 0) {
//            //println!("right {} {}: {:?}", i, k, seatmap[i][k]);
//        }
//        if k != j && seatmap[i][k] == SeatState::OCCUPIED {
//            count += 1;
//            break;
//        }
//        if k != j && seatmap[i][k] == SeatState::EMPTY {
//            break;
//        }
//    }
//    // look left
//    for k in (0..j+1).rev() {
//        if (i == 2 && j == 0) {
//            //println!("left {} {}: {:?}", i, k, seatmap[i][k]);
//        }
//        if k != j && seatmap[i][k] == SeatState::OCCUPIED {
//            count += 1;
//            break;
//        }
//        if k != j && seatmap[i][k] == SeatState::EMPTY {
//            break;
//        }
//
//    }
//    // look down
//    for k in i..h {
//        if (i == 2 && j == 0) {
//            //println!("down {} {}: {:?}", k, j, seatmap[k][j]);
//        }
//        if k != i && seatmap[k][j] == SeatState::OCCUPIED {
//            count += 1;
//            break;
//        }
//        if k != i && seatmap[k][j] == SeatState::EMPTY {
//            break;
//        }
//    }
//    // look up
//    for k in (0..i+1).rev() {
//        if (i == 2 && j == 0) {
//            //println!("up {} {}: {:?}", k, j, seatmap[k][j]);
//        }
//        if k != i && seatmap[k][j] == SeatState::OCCUPIED {
//            count += 1;
//            break;
//        }
//        if k != i && seatmap[k][j] == SeatState::EMPTY {
//            break;
//        }
//    }
//    // look top-right
//    for (k, l) in (0..i+1).rev().zip(j..w) {
//        if (i == 2 && j == 0) {
//            //println!("tr {} {}: {:?}", k, l, seatmap[k][l]);
//        }
//        if k != i && l != j && seatmap[k][l] == SeatState::OCCUPIED {
//            count += 1;
//            break;
//        }
//        if k != i && l != j && seatmap[k][l] == SeatState::EMPTY {
//            break;
//        }
//    }
//    // look bottom right
//    for (k, l) in (i..h).zip(j..w) {
//        if (i == 2 && j == 0) {
//            //println!("br {} {}: {:?}", k, l, seatmap[k][l]);
//        }
//        if k != i && l != j && seatmap[k][l] == SeatState::OCCUPIED {
//            count += 1;
//            break;
//        }
//        if k != i && l != j && seatmap[k][l] == SeatState::EMPTY {
//            break;
//        }
//    }
//    // look bottom  left
//    for (k, l) in (i..h).zip((0..j+1).rev()) {
//        if (i == 2 && j == 0) {
//            //println!("bl {} {}: {:?}", k, l, seatmap[k][l]);
//        }
//        if k != i && l != j && seatmap[k][l] == SeatState::OCCUPIED {
//            count += 1;
//            break;
//        }
//        if k != i && l != j && seatmap[k][l] == SeatState::EMPTY {
//            break;
//        }
//    }
//    // look top left
//    for (k, l) in (0..i+1).rev().zip((0..j+1).rev()) {
//        if (i == 2 && j == 0) {
//            //println!("tl {} {}: {:?}", k, l, seatmap[k][l]);
//        }
//        if k != i && l != j && seatmap[k][l] == SeatState::OCCUPIED {
//            count += 1;
//            break;
//        }
//        if k != i && l != j && seatmap[k][l] == SeatState::EMPTY {
//            break;
//        }
//    }
//
//    if (i == 2 && j == 0) {
//        //println!("{} {}: {}", i, j, count);
//    }
//    return count;
//}



fn step(seatmap: &SeatMap) -> Option<SeatMap> {
    let mut changes = 0;
    let next = seatmap.seats.iter().enumerate().map(|(ii, seat)| {
        let i = ii as i32;
        let p = Point {x: i % seatmap.width, y: i / seatmap.width };
        match seat {
            SeatState::FLOOR => SeatState::FLOOR,
            SeatState::EMPTY => {
                if seatmap.count_adjacent(p) == 0 {
                    changes += 1;
                    SeatState::OCCUPIED
                } else {
                    SeatState::EMPTY
                }
            },
            SeatState::OCCUPIED=> {
                if seatmap.count_adjacent(p) >= 4 {
                    changes += 1;
                    SeatState::EMPTY
                } else {
                    SeatState::OCCUPIED
                }
            }
        }
    }).collect();

    println!("Step changed {}", changes);

    if changes == 0 {
        None
    } else {
        Some(SeatMap { 
                height: seatmap.height,
                width: seatmap.width,
                seats: next
            })
    }
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let chars: Vec<Vec<char>> = buf.lines()
                                   .map(|l| l.unwrap().chars().collect::<Vec<char>>())
                                   .collect();

    let width = chars[0].len() as i32;
    let height = chars.len() as i32;
    let seats:Vec<SeatState> = chars.iter()
                                    .flatten()
                                    .map(|c| SeatState::from_char(&c))
                                    .collect();

    let mut seatmap = SeatMap { width: width, height: height, seats: seats };

    println!("Start: There are {} occupied",  seatmap.count_occupied());

    let  mut i = 0;

    loop {
        //println!("{}\n", seatmap);
        let next = step(&seatmap);
        if next.is_none() {
            break;
        } else {
            seatmap = next.unwrap();
        }
        //i += 1;
        //if (i == 10) { break; }
    }

    println!("Part 1: There are {} occupied seats",  seatmap.count_occupied());
}
