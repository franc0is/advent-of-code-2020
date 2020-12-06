use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

struct Position {
    x: usize,
    y: usize,
}


fn solve_slope(tree_map: &Vec<Vec<bool>>, slope: &Position) -> u32 {
    let map_height = tree_map.len();
    let map_width = tree_map[0].len();
    let mut position = Position { x: 0, y: 0 };
    let mut num_trees = 0;

    while position.y < map_height {
        if tree_map[position.y][position.x] { num_trees = num_trees + 1; }
        position.x = (position.x + slope.x) % map_width;
        position.y = position.y + slope.y;
    }

    return num_trees
}


fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let tree_map: Vec<Vec<bool>> = buf.lines()                          // all lines in file
                                      .map(|l| l.unwrap().chars()       // map line -> chars
                                                .map(|c| {              // map chars -> bools
                                                    match c {
                                                        '#' => true,
                                                        '.' => false,
                                                        _ => panic!()
                                                    }
                                                }).collect()            // collect into inner Vec
                                      ).collect();                      // collect into outer Vec

    let slopes  = [
        Position{ x: 1, y: 1},
        Position{ x: 3, y: 1},
        Position{ x: 5, y: 1},
        Position{ x: 7, y: 1},
        Position{ x: 1, y: 2}
    ];
    let mut result = 1;
    for slope in slopes.iter() {
        result = result * solve_slope(&tree_map, slope);
    }

    println!("Result is {}", result);
}
