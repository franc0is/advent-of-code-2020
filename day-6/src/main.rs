use std::fs;
use std::collections::HashSet;

fn part_one(groups: &Vec<&str>) -> usize {
    let mut result = 0;
    for group in groups {
        let chars = group.chars();
        let mut seen_char = HashSet::new();
        for c in chars {
            match c {
                '\n' => continue,
                _ => { seen_char.insert(c); }
            }
        }
        result = result + seen_char.len();
    }

    return result;
}

fn part_two(groups: &Vec<&str>) -> usize {
    let mut result = 0;
    for group in groups {
        let people: Vec<&str> = group.split('\n').collect();
        let mut shared_chars: Option<HashSet<char>> = None;
        for person in people {
            let chars_set: HashSet<char> = person.chars().into_iter().collect();
            if shared_chars == None {
                shared_chars = Some(chars_set);
            } else {
                shared_chars = Some(shared_chars.unwrap().intersection(&chars_set).cloned().collect());
            }
        }
        result = result + shared_chars.unwrap().len();
    }

    return result;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let groups: Vec<&str> = input.split("\n\n").collect();
    println!("Part one, result is {}", part_one(&groups));
    println!("Part one, result is {}", part_two(&groups));
}
