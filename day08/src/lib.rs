use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();
        let arg = parts.next().unwrap().parse().unwrap();
        match op {
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            "nop" => Instruction::Nop(arg),
            n => panic!("Unexpected program instruction variant: {}", n),
        }
    }
}

#[derive(Debug)]
pub enum ProgramError {
    // Loop detected, at the given instruction
    LoopDetected(i32),
}

#[derive(Clone, Debug, Default)]
pub struct Computer {
    pub program: Vec<Instruction>,
    pub pc: i32,
    pub acc: i32,
    pub visited_pcs: HashSet<i32>,
}

impl Computer {
    pub fn new(input: &str) -> Self {
        let program = input.lines().map(From::from).collect();
        Self {
            program,
            ..Default::default()
        }
    }

    pub fn run_until_first_loop(&mut self) -> i32 {
        while !self.visited_pcs.contains(&self.pc) {
            self.visited_pcs.insert(self.pc);
            self.step();
        }
        self.acc
    }

    /// Run until completion, returning the final accumulator value
    pub fn run(&mut self) -> Result<i32, ProgramError> {
        while (self.pc as usize) < self.program.len() {
            if self.visited_pcs.contains(&self.pc) {
                return Err(ProgramError::LoopDetected(self.pc));
            }
            self.visited_pcs.insert(self.pc);
            self.step();
        }
        Ok(self.acc)
    }

    fn step(&mut self) {
        let instruction = self.program[self.pc as usize];
        match instruction {
            Instruction::Acc(n) => {
                self.acc += n;
                self.pc += 1;
            }
            Instruction::Jmp(n) => self.pc += n,
            Instruction::Nop(_) => self.pc += 1,
        }
    }
}
