use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{prelude::*, BufReader}
};

use regex::Regex;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
enum OpType {
    NONE,
    ADD,
    MUL,
}

impl OpType {
    fn from_char(c: char) -> Self {
        match c {
            '+' => OpType::ADD,
            '*' => OpType::MUL,
            _ => panic!("Invalid optype char {}", c),
        }
    }
}

#[derive(Debug)]
struct Operation {
    args: Vec<u64>,
    t: OpType,
}

impl Operation {
    fn new() -> Self {
        return Operation { args: Vec::new(), t: OpType::NONE };
    }
}

#[derive(Debug)]
struct Calculator {
    ops: VecDeque<Operation>,
    result: u64,
}

impl Calculator {
    fn new() -> Self {
        let mut inst = Calculator { ops: VecDeque::new(), result: 0 };
        return inst;
    }

    fn compute(&mut self) -> u64 {
        print!("COMPUTE ");

        let op = &self.ops[0];

        let is_mul = op.t == OpType::MUL;

        let result = match op.t {
            OpType::ADD => op.args[0] + op.args[1],
            OpType::MUL => op.args[0] * op.args[1],
            OpType::NONE => panic!("Computing NONE"),
        };

        if is_mul {
            self.ops.pop_front();
        } else {
            self.ops[0] = Operation::new();
        }

        if self.ops.is_empty() {
            self.ops.push_front(Operation::new());
        }

        return result;
    }

    fn pop(&mut self) -> () {
        print!("POP ");
        let mut result = self.result;

        if self.ops[0].args.len() == 1 {
            result = self.ops[0].args[0];
        }
        self.ops.pop_front();

        if !self.ops.is_empty() {
            self.ops[0].args.push(result);
        } else {
            self.ops.push_front(Operation::new());
            self.ops[0].args.push(result);
        }
    }

    fn  push(&mut self) -> () {
        if self.ops.is_empty() {
            return;
        }
        print!("PUSH ");
        self.ops.push_front(Operation::new());
    }

    fn set_arg(&mut self, c: char) -> () {
        print!("SETARG ");
        if self.ops.is_empty() {
            print!("NEW ");
            self.ops.push_front(Operation::new());
        }
        let n: u64 = c.to_digit(10).unwrap().into();
        self.ops[0].args.push(n);
    }

    fn set_op_type(&mut self, c: char) -> () {
        print!("SETOPT ");
        assert!(self.ops[0].t == OpType::NONE);
        self.ops[0].t = OpType::from_char(c);
        if self.ops[0].t == OpType::MUL {
            self.push();
        }
    }
}

fn compute_line(line: &str) -> u64 {
    let mut calc = Calculator::new();
    for c in line.chars() {
        print!("{} : ", c);
        match c {
            ' ' => continue,
            '(' => calc.push(),
            ')' => calc.pop(),
            '+' | '*' => calc.set_op_type(c),
            '0'..='9' => calc.set_arg(c),
            _ => panic!("Unexpected char  {}", c),
        };

        while !calc.ops.is_empty() && calc.ops[0].args.len() == 2 {
            calc.result = calc.compute();
            calc.ops[0].args.push(calc.result);
        }

        print!("\t\t{:?}\n", calc.ops);
    }

    println!("------ Finishing ------");
    if !calc.ops.is_empty() {
        calc.pop();
        while !calc.ops.is_empty() && calc.ops[0].args.len() == 2 {
            calc.result = calc.compute();
            calc.ops[0].args.push(calc.result);
        }
        print!("\t\t{:?}\n", calc.ops);
    }

    return calc.result;
}


fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let input = BufReader::new(f).lines().map(|l| l.unwrap());

    let mut output = 0;
    for line in input {
        let o = compute_line(&line);
        output += o;
        println!("{} => {} ({})", line, o, output);
    }

    println!("Output: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let line = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(compute_line(line), 231);
    }

    #[test]
    fn test_parens_1() {
        let line = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(compute_line(line), 51);
    }

    #[test]
    fn test_parens_2() {
        let line = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(compute_line(line), 23340);
    }

    #[test]
    fn test_parens_3() {
        let line = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(compute_line(line), 669060);
    }

    #[test]
    fn test_parens_4() {
        let line = "5 + (8 * 3 + 9 + 3 * 4 * 3";
        assert_eq!(compute_line(line), 1445);
    }

    #[test]
    fn test_parens_5()  {
        let line = "2 * 3 + (4 * 5)";
        assert_eq!(compute_line(line), 46);
    }

    #[test]
    fn test_more() {
        let line = "(6 * (2 * 7 * 3)) + 5";
        assert_eq!(compute_line(line), 257);
    }

}
