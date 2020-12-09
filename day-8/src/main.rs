use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use regex::Regex;
use std::collections::{HashSet};

#[derive(PartialEq, Debug)]
enum VMState {
    PatchRestart,
    PatchPending,
    Patched,
    Running,
}

struct VM {
    pc: i32,
    acc: i32,
    state: VMState,
    seen_pcs: HashSet<i32>,
    patched_pc: i32,
    patched_acc: i32,
    re: Regex,
}

impl VM {
    fn apply_patch(&mut self) {
        self.patched_pc = self.pc;
        self.patched_acc = self.acc;
        self.state = VMState::Patched;
    }

    fn enable_patch(&mut self) {
        self.state = VMState::PatchPending;
    }

    fn execute_instruction(&mut self, opcode: &str, imm: i32) {
        match opcode {
            "nop" => {
                if self.state == VMState::PatchPending {
                    self.apply_patch();
                    self.pc = self.pc + imm;
                } else {
                    self.pc = self.pc + 1;
                }
            },
            "jmp" => {
                if self.state == VMState::PatchPending {
                    self.apply_patch();
                    self.pc = self.pc + 1;
                } else {
                    self.pc = self.pc + imm;
                }
            }
            "acc" => {
                self.acc = self.acc + imm;
                self.pc = self.pc + 1;
            }
            _ => {
                panic!("Invalid opcode {}", opcode);
            }
        }
        if self.state == VMState::PatchRestart {
            self.state = VMState::PatchPending;
        }
    }

    fn check_infinite_loop(&mut self) -> bool {
        if self.seen_pcs.contains(&self.pc) {
            return true;
        } else {
            self.seen_pcs.insert(self.pc);
            return false;
        }
    }

    fn exec(&mut self, program: &Vec<String>) {
        while self.pc < program.len() as i32 {
            if !self.check_infinite_loop() {
                self.execute_line(&program[self.pc as usize]);
            } else if self.state == VMState::Patched {
                self.reset_at_patch()
            } else {
                println!("Infinite loop, stopping ...");
                return;
            }
        }
    }

    fn reset_at_patch(&mut self) {
        self.pc = self.patched_pc;
        self.acc = self.patched_acc;
        self.seen_pcs.clear();
        self.state = VMState::PatchRestart;
    }

    fn execute_line(&mut self, line: &String) {
        let cap = self.re.captures(&line).unwrap();
        self.execute_instruction(&cap[1], cap[2].parse().unwrap());
    }

    fn new() -> VM {
        return VM {
            pc: 0,
            acc: 0,
            state: VMState::Running,
            re: Regex::new(r"^(\w+) ([+-]\d+)$").unwrap(),
            seen_pcs: HashSet::new(),
            patched_pc: 0,
            patched_acc: 0,
        };
    }
}


fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let buf = BufReader::new(f);
    let program: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

    // Part 1
    let mut vm = VM::new();
    vm.exec(&program);
    println!("Part 1 Program terminated. PC {} Acc {}", vm.pc, vm.acc);

    // Part 2
    vm = VM::new();
    vm.enable_patch();
    vm.exec(&program);
    println!("Part 2 Program terminated. PC {} Acc {}", vm.pc, vm.acc);
}
