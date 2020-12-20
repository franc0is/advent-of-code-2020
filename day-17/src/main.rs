use std::{
    ops::Add,
    fs::File,
    io::{prelude::*, BufReader}
};

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum CubeState {
    ON,
    OFF,
}

impl CubeState {
    fn from_char(c: &char) -> CubeState {
        match  c {
            '.' => CubeState::OFF,
            '#' => CubeState::ON,
            _ => panic!("Invalid cube input {}", c)
        }
    }
}

impl std::fmt::Display for CubeState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CubeState::OFF => write!(f, "."),
            CubeState::ON => write!(f, "#")
        }
    }
}

struct Dimension {
    width: i32,
    length: i32,
    height: i32,
    fourth: i32,
    cubes: Vec<CubeState>,
}

impl Dimension {
    const ADJACENT_MAP: [Point;80] = [
        Point { x: -1, y: -1, z: -1, w: -1 }, Point { x:  0, y: -1, z: -1, w: -1 }, Point { x:  1, y: -1, z: -1, w: -1 },
        Point { x: -1, y:  0, z: -1, w: -1 }, Point { x:  0, y:  0, z: -1, w: -1 }, Point { x:  1, y:  0, z: -1, w: -1 },
        Point { x: -1, y:  1, z: -1, w: -1 }, Point { x:  0, y:  1, z: -1, w: -1 }, Point { x:  1, y:  1, z: -1, w: -1 },

        Point { x: -1, y: -1, z: 0, w: -1 }, Point { x:  0, y: -1, z: 0, w: -1 }, Point { x:  1, y: -1, z: 0, w: -1 },
        Point { x: -1, y:  0, z: 0, w: -1 }, Point { x:  0, y:  0, z: 0, w: -1 }, Point { x:  1, y:  0, z: 0, w: -1 },
        Point { x: -1, y:  1, z: 0, w: -1 }, Point { x:  0, y:  1, z: 0, w: -1 }, Point { x:  1, y:  1, z: 0, w: -1 },

        Point { x: -1, y: -1, z: 1, w: -1 }, Point { x:  0, y: -1, z: 1, w: -1 }, Point { x:  1, y: -1, z: 1, w: -1 },
        Point { x: -1, y:  0, z: 1, w: -1 }, Point { x:  0, y:  0, z: 1, w: -1 }, Point { x:  1, y:  0, z: 1, w: -1 },
        Point { x: -1, y:  1, z: 1, w: -1 }, Point { x:  0, y:  1, z: 1, w: -1 }, Point { x:  1, y:  1, z: 1, w: -1 },


        Point { x: -1, y: -1, z: -1, w: 0 }, Point { x:  0, y: -1, z: -1, w: 0 }, Point { x:  1, y: -1, z: -1, w: 0 },
        Point { x: -1, y:  0, z: -1, w: 0 }, Point { x:  0, y:  0, z: -1, w: 0 }, Point { x:  1, y:  0, z: -1, w: 0 },
        Point { x: -1, y:  1, z: -1, w: 0 }, Point { x:  0, y:  1, z: -1, w: 0 }, Point { x:  1, y:  1, z: -1, w: 0 },

        Point { x: -1, y: -1, z: 0, w: 0 }, Point { x:  0, y: -1, z: 0, w: 0 }, Point { x:  1, y: -1, z: 0, w: 0 },
        Point { x: -1, y:  0, z: 0, w: 0 },                                     Point { x:  1, y:  0, z: 0, w: 0 },
        Point { x: -1, y:  1, z: 0, w: 0 }, Point { x:  0, y:  1, z: 0, w: 0 }, Point { x:  1, y:  1, z: 0, w: 0 },

        Point { x: -1, y: -1, z: 1, w: 0 }, Point { x:  0, y: -1, z: 1, w: 0 }, Point { x:  1, y: -1, z: 1, w: 0 },
        Point { x: -1, y:  0, z: 1, w: 0 }, Point { x:  0, y:  0, z: 1, w: 0 }, Point { x:  1, y:  0, z: 1, w: 0 },
        Point { x: -1, y:  1, z: 1, w: 0 }, Point { x:  0, y:  1, z: 1, w: 0 }, Point { x:  1, y:  1, z: 1, w: 0 },


        Point { x: -1, y: -1, z: -1, w: 1 }, Point { x:  0, y: -1, z: -1, w: 1 }, Point { x:  1, y: -1, z: -1, w: 1 },
        Point { x: -1, y:  0, z: -1, w: 1 }, Point { x:  0, y:  0, z: -1, w: 1 }, Point { x:  1, y:  0, z: -1, w: 1 },
        Point { x: -1, y:  1, z: -1, w: 1 }, Point { x:  0, y:  1, z: -1, w: 1 }, Point { x:  1, y:  1, z: -1, w: 1 },

        Point { x: -1, y: -1, z: 0, w: 1 }, Point { x:  0, y: -1, z: 0, w: 1 }, Point { x:  1, y: -1, z: 0, w: 1 },
        Point { x: -1, y:  0, z: 0, w: 1 }, Point { x:  0, y:  0, z: 0, w: 1 }, Point { x:  1, y:  0, z: 0, w: 1 },
        Point { x: -1, y:  1, z: 0, w: 1 }, Point { x:  0, y:  1, z: 0, w: 1 }, Point { x:  1, y:  1, z: 0, w: 1 },

        Point { x: -1, y: -1, z: 1, w: 1 }, Point { x:  0, y: -1, z: 1, w: 1 }, Point { x:  1, y: -1, z: 1, w: 1 },
        Point { x: -1, y:  0, z: 1, w: 1 }, Point { x:  0, y:  0, z: 1, w: 1 }, Point { x:  1, y:  0, z: 1, w: 1 },
        Point { x: -1, y:  1, z: 1, w: 1 }, Point { x:  0, y:  1, z: 1, w: 1 }, Point { x:  1, y:  1, z: 1, w: 1 },
    ];

    fn get(&self, p: Point) -> Option<CubeState> {
        if p.x < 0 || p.x >= self.width ||
           p.y < 0 || p.y >= self.length ||
           p.z < 0 || p.z >= self.height ||
           p.w < 0 || p.w >= self.fourth {
            // out of bounds
            return None;
        }

        let i = (p.w * self.width * self.height * self.length +
                 p.z * self.width * self.length +
                 p.y * self.width +
                 p.x) as usize;
        Some(self.cubes[i])
    }

    fn get_adjacent(&self, p: Point) -> Vec<CubeState> {
        return Self::ADJACENT_MAP.iter()
                                 .map(|adjacent| self.get(*adjacent + p))
                                 .filter(|opt| opt.is_some())
                                 .map(|opt| opt.unwrap())
                                 .collect();
    }

    fn count_adjacent(&self, p: Point) -> usize {
        let c = self.get_adjacent(p).iter().fold(0, |acc, adjacent| {
            match adjacent {
                CubeState::OFF => {
                    acc
                },
                CubeState::ON => {
                    acc + 1
                }
            }
        });

        //println!("{} {} {} : {}", p.x, p.y, p.z, c);
        c

    }

    fn count_on(&self) -> usize {
        self.cubes.iter().fold(0, |acc, s| {
            if *s == CubeState::ON {
                acc + 1
            } else {
                acc
            }
        })
    }
}


impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut z = 1;
        println!("z: {}\n", 0);
        for (i, cube) in self.cubes.iter().enumerate() {
            if i != 0 && i % (self.width as usize) == 0 {
                write!(f, "\n");
            }
            if i != 0 && i % (self.width as usize * self.length as usize) == 0 {
                write!(f, "\n\nz: {}\n", z);
                z += 1;
            }
            write!(f, "{}", cube);
        }
        Ok(())
    }
}

fn step(dimension: &Dimension) -> Dimension {
    let next = dimension.cubes.iter().enumerate().map(|(ii, cube)| {
        let i = ii as i32;
        let p = Point {
            x: i % dimension.width,
            y: i / dimension.width % dimension.length,
            z: i / (dimension.width * dimension.length) % dimension.height,
            w: i / (dimension.length * dimension.width * dimension.height),
        };
        match cube {
            CubeState::OFF => {
                if dimension.count_adjacent(p) == 3 {
                    //println!("flipping {} from ON to OFF", p);
                    CubeState::ON
                } else {
                    CubeState::OFF
                }
            },
            CubeState::ON=> {
                let adjacent = dimension.count_adjacent(p);
                if adjacent == 2 || adjacent == 3 {
                    CubeState::ON
                } else {
                    //println!("flipping {} from ON to OFF", p);
                    CubeState::OFF
                }
            }
        }
    }).collect();

    Dimension { 
        length: dimension.length,
        width: dimension.width,
        height: dimension.height,
        fourth: dimension.fourth,
        cubes: next
    }
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let chars: Vec<Vec<char>> = buf.lines()
                                   .map(|l| l.unwrap().chars().collect::<Vec<char>>())
                                   .collect();

    let width = 12 + chars[0].len() as i32;
    let length = 12 + chars.len() as i32;
    let height = 12 + 1 as i32;
    let fourth = 12 + 1 as i32;
    let input: Vec<Vec<CubeState>>  = chars.iter()
                     .map(|l| l.iter().map(|c| CubeState::from_char(&c)).collect())
                     .collect();

    let mut cubes: Vec<CubeState> = vec![CubeState::OFF; (width * length * height * fourth) as usize];

    for (i, l) in input.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            let x = j + 6;
            let y = i + 6;
            let z = 6;
            let w = 6;
            let cube_idx = w * (width * length * height) as usize +
                           z * (width * length) as usize +
                           y * width as usize +
                           x;
            cubes[cube_idx] = *c;
        }
    }

    let mut dimension = Dimension { width: width, length: length, height: height, fourth: fourth, cubes: cubes };

    println!("Start: There are {} ON",  dimension.count_on());
    //println!("{}\n", dimension);

    let  mut i = 0;

    loop {
        dimension = step(&dimension);
        //println!("{}\n", dimension);
        i += 1;
        if (i == 6) { break; }
    }

    println!("Part 2: There are {} ON",  dimension.count_on());
}
