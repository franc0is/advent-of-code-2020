use std::{
    fs::File,
    io::{prelude::*, BufReader}
};



#[derive(PartialEq, Debug, Clone)]
enum SeatState {
    FLOOR,
    OCCUPIED,
    EMPTY,
}

type SeatMap = Vec<Vec<SeatState>>;

fn char_to_seat_state(c: &char) -> SeatState {
    match  c {
        'L' => SeatState::EMPTY,
        '.' => SeatState::FLOOR,
        '#' => SeatState::OCCUPIED,
        _ => panic!("Invalid input {}", c)
    }
}

fn count_state(seatmap: &SeatMap, state: SeatState) -> usize {
    return seatmap.iter().flatten().fold(0, |acc, s| {
        if *s == state {
            acc + 1
        } else {
            acc
        }
    });
}

//fn seat_map_to_matrix(seatmap: &SeatMap) -> Vec<Vec<usize>> {
//    return seatmap.clone().iter().map(|row| row.iter().map(|seat| match seat {
//        SeatState::EMPTY | SeatState::FLOOR => 0,
//        SeatState::OCCUPIED => 1
//    }).collect()).collect();
//}

fn count_adjacent_part1(seatmap: &SeatMap, i: usize, j: usize) -> usize {
    let h = seatmap.len() - 1;
    let w = seatmap[0].len() - 1;
    let mut adjacent: Vec<&SeatState> = Vec::new();

    if i > 0 && j > 0 {
        adjacent.push(&seatmap[i - 1][j - 1]);
    }
    if i < h && j < w {
        adjacent.push(&seatmap[i + 1][j + 1]);
    }
    if i > 0 && j < w {
        adjacent.push(&seatmap[i - 1][j + 1]);
    }
    if i < h && j > 0 {
        adjacent.push(&seatmap[i + 1][j - 1]);
    }
    if i > 0 {
        adjacent.push(&seatmap[i - 1][j]);
    }
    if j > 0 {
        adjacent.push(&seatmap[i][j - 1]);
    }
    if i < h {
        adjacent.push(&seatmap[i + 1][j]);
    }
    if j < w {
        adjacent.push(&seatmap[i][j + 1]);
    }

    return adjacent.iter().fold(0, |acc, seat| match seat {
        SeatState::EMPTY | SeatState::FLOOR => {
            acc
        },
        SeatState::OCCUPIED => {
            acc + 1
        }
    });
}

fn count_adjacent_part2(seatmap: &SeatMap, i: usize, j: usize) -> usize {
    let h = seatmap.len();
    let w = seatmap[0].len();
    let mut count = 0;

    // look right
    for k in j..w {
        if (i == 2 && j == 0) {
            //println!("right {} {}: {:?}", i, k, seatmap[i][k]);
        }
        if k != j && seatmap[i][k] == SeatState::OCCUPIED {
            count += 1;
            break;
        }
        if k != j && seatmap[i][k] == SeatState::EMPTY {
            break;
        }
    }
    // look left
    for k in (0..j+1).rev() {
        if (i == 2 && j == 0) {
            //println!("left {} {}: {:?}", i, k, seatmap[i][k]);
        }
        if k != j && seatmap[i][k] == SeatState::OCCUPIED {
            count += 1;
            break;
        }
        if k != j && seatmap[i][k] == SeatState::EMPTY {
            break;
        }

    }
    // look down
    for k in i..h {
        if (i == 2 && j == 0) {
            //println!("down {} {}: {:?}", k, j, seatmap[k][j]);
        }
        if k != i && seatmap[k][j] == SeatState::OCCUPIED {
            count += 1;
            break;
        }
        if k != i && seatmap[k][j] == SeatState::EMPTY {
            break;
        }
    }
    // look up
    for k in (0..i+1).rev() {
        if (i == 2 && j == 0) {
            //println!("up {} {}: {:?}", k, j, seatmap[k][j]);
        }
        if k != i && seatmap[k][j] == SeatState::OCCUPIED {
            count += 1;
            break;
        }
        if k != i && seatmap[k][j] == SeatState::EMPTY {
            break;
        }
    }
    // look top-right
    for (k, l) in (0..i+1).rev().zip(j..w) {
        if (i == 2 && j == 0) {
            //println!("tr {} {}: {:?}", k, l, seatmap[k][l]);
        }
        if k != i && l != j && seatmap[k][l] == SeatState::OCCUPIED {
            count += 1;
            break;
        }
        if k != i && l != j && seatmap[k][l] == SeatState::EMPTY {
            break;
        }
    }
    // look bottom right
    for (k, l) in (i..h).zip(j..w) {
        if (i == 2 && j == 0) {
            //println!("br {} {}: {:?}", k, l, seatmap[k][l]);
        }
        if k != i && l != j && seatmap[k][l] == SeatState::OCCUPIED {
            count += 1;
            break;
        }
        if k != i && l != j && seatmap[k][l] == SeatState::EMPTY {
            break;
        }
    }
    // look bottom  left
    for (k, l) in (i..h).zip((0..j+1).rev()) {
        if (i == 2 && j == 0) {
            //println!("bl {} {}: {:?}", k, l, seatmap[k][l]);
        }
        if k != i && l != j && seatmap[k][l] == SeatState::OCCUPIED {
            count += 1;
            break;
        }
        if k != i && l != j && seatmap[k][l] == SeatState::EMPTY {
            break;
        }
    }
    // look top left
    for (k, l) in (0..i+1).rev().zip((0..j+1).rev()) {
        if (i == 2 && j == 0) {
            //println!("tl {} {}: {:?}", k, l, seatmap[k][l]);
        }
        if k != i && l != j && seatmap[k][l] == SeatState::OCCUPIED {
            count += 1;
            break;
        }
        if k != i && l != j && seatmap[k][l] == SeatState::EMPTY {
            break;
        }
    }

    if (i == 2 && j == 0) {
        //println!("{} {}: {}", i, j, count);
    }
    return count;
}



fn step(seatmap: &SeatMap) -> (usize, SeatMap) {
    let h = seatmap.len();
    let w = seatmap[0].len();
    let mut changes = 0;
    let mut nm: SeatMap = vec![vec![SeatState::FLOOR; w]; h];
    for i in 0..h {
        for j in 0..w {
            let seat = &seatmap[i][j];
            if *seat == SeatState::FLOOR {
                continue;
            } else {
                let c = count_adjacent_part2(seatmap, i, j);
                //println!("count {}", c);
                if *seat == SeatState::EMPTY && c == 0 {
                    nm[i][j] = SeatState::OCCUPIED;
                    changes += 1
                }  else if *seat == SeatState::OCCUPIED && c >= 5 {
                    nm[i][j] = SeatState::EMPTY;
                    changes += 1
                } else {
                    nm[i][j] = seatmap[i][j].clone();
                }
            }
        }
    }

    return (changes, nm);
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let mut seatmap: SeatMap = buf.lines()
                                  .map(|l| l.unwrap().chars().map(|c| char_to_seat_state(&c)).collect())
                                  .collect();

    println!("Start: There are {} seats",  count_state(&seatmap, SeatState::EMPTY));
    println!("Start: There are {} occupied",  count_state(&seatmap, SeatState::OCCUPIED));

    //let mut n = 0;
    //println!("{:?}", seatmap);
    //let (n, s) = step(&seatmap);
    //seatmap = s;
    //println!(">>>>>");
    //println!("{:?}", seatmap);
    //let (n, s) = step(&seatmap);
    //seatmap = s;
    loop {
        let (n, s) = step(&seatmap);
        seatmap = s;
        //println!("{:?}\n", seatmap);
        if (n == 0) {
            break;
        }
    }

    println!("Part 1: There are {} occupied seats",  count_state(&seatmap, SeatState::OCCUPIED));
}
