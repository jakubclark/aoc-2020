use day08::{Computer, Instruction, ProgramError};

fn solve_part1(input: &str) -> i32 {
    Computer::new(input).run_until_first_loop()
}

fn solve_part2(input: &str) -> i32 {
    let computer = Computer::new(input);
    let instructions = computer.program;

    let mut i = 0;
    let max_i = instructions.len();

    while i < max_i {
        let mut program = instructions.to_owned();
        match program[i] {
            Instruction::Jmp(n) => program[i] = Instruction::Nop(n),
            Instruction::Nop(n) => program[i] = Instruction::Jmp(n),
            _ => {
                i += 1;
                continue;
            }
        }
        let mut computer = Computer {
            program,
            ..Default::default()
        };
        if let Err(ProgramError::LoopDetected(_)) = computer.run() {
            i += 1;
            continue;
        } else {
            return computer.acc;
        }
    }

    unreachable!()
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1 solution: {}", part1);
    let part2 = solve_part2(input);
    println!("Part 2 solution: {}", part2);
}

#[test]
fn part1_works() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let part1 = solve_part1(input);
    assert_eq!(5, part1);
}

#[test]
fn part2_works() {
    let input = "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";
    let part2 = solve_part2(input);
    assert_eq!(8, part2);
}
