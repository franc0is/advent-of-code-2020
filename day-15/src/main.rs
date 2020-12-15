use std::collections::HashMap;

fn get_nth(input: &Vec<usize>, n: usize) -> usize {
    let mut m: HashMap<usize, usize> = HashMap::new();
    let mut number: usize = 0;

    for i in 0..n-1 {
        if i < input.len() {
            number = input[i];
        }

        let next = match m.get(&number) {
            None => {
                0
            },
            Some(x) => {
                i - x
            },
        };
        m.insert(number, i);
        number = next;
    }

    return number;
}

fn main() {
    let input: Vec<usize> = vec!(20,0,1,11,6,3);
    println!("Part 1: {}", get_nth(&input, 2020));
    println!("Part 2: {}", get_nth(&input, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps() {
        let input: Vec<usize> = vec!(0,3,6);
        let values = [0, 3, 3, 1, 0, 4, 0];
        for i in 0..7 {
            println!("Looking for {}, expecting {}", i, values[i]);
            assert_eq!(get_nth(&input, i + 4), values[i]);
        }
    }


    #[test]
    fn test_sequence() {
        let input: Vec<usize> = vec!(1,3,2);
        assert_eq!(get_nth(&input, 2020), 1);
    }

    #[test]
    fn test_large_sequence() {
        let input: Vec<usize> = vec!(0,3,6);
        assert_eq!(get_nth(&input, 30000000), 175594);
    }
}
