use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use regex::Regex;
use std::collections::{HashMap,HashSet};

#[derive(Debug, Clone)]
struct Bag {
    s: String,
    n: u32,
}

fn add_containers(set: &mut HashSet<String>, map: &HashMap<String, Vec<String>>, s: String) {
    if set.contains(&s) {
        return;
    }
    let v_o = map.get(&s);
    set.insert(s.clone());
    match v_o {
        Some(v) => {
            for ss in v {
                println!("{}", &ss);
                add_containers(set, map, ss.to_string());
            }
        }
        None => {
            return;
        }
    }
}

fn add_contained(map: &HashMap<String, Vec<Bag>>, s: &String, n: u32) -> u32 {
    let v_o  = map.get(s);
    match v_o {
        Some(v) => {
            println!("{} {}", n, s);
            let mut nn: u32 = n;
            for b in v {
                nn = nn + add_contained(map, &b.s, b.n * n);
            }
            return nn;
        }
        None => {
            println!("{} {}", n, s);
            return n;
        }
    }
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let rules = buf.lines().map(|l| l.unwrap());

    let bag_re = Regex::new(r"^(\w+ \w+) bags contain").unwrap();
    let contents_re = Regex::new(r"(\d{1}) (\w+ \w+) bag").unwrap();

    let mut contained_by_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut contains_map: HashMap<String, Vec<Bag>> = HashMap::new();
    for rule in rules {
        let cap = bag_re.captures(&rule).unwrap();
        let container = String::from(&cap[1]);
        let mut vv: Vec<Bag> = Vec::new();

        //println!("Container: {}", container);
        let cap_iter = contents_re.captures_iter(&rule);
        for cap in cap_iter {
            let content = String::from(&cap[2]);
            let n: u32 = cap[1].parse().unwrap();
            vv.push(Bag { s: content.clone(), n: n });
            let mut v: Vec<String>;
            if !contained_by_map.contains_key(&content) {
                v = Vec::new();
            } else {
                v = contained_by_map.get(&content).unwrap().to_vec();
            }
            v.push(container.clone());
            contained_by_map.insert(content.clone(), v);
            contains_map.insert(container.clone(), vv.clone());
        }
    }

    //println!("Map: {:?}", contains_map);

    // part 1
    println!("##### PART 1 ######\n");
    let mut can_contain_set: HashSet<String> = HashSet::new();
    let look_for = String::from("shiny gold");
    add_containers(&mut can_contain_set, &contained_by_map, look_for.clone());

    println!("\n====> Result 1: {}", can_contain_set.len() - 1);
    println!("\n\n##### PART 2 #####\n");

    // part 2
    let n = add_contained(&contains_map, &look_for, 1);

    println!("\n====> Result 2: {}", n - 1);


}
