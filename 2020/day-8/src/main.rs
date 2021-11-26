// --- Day 8: Handheld Halting ---
// Your flight to the major airline hub reaches cruising altitude without incident. While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.

// Their handheld game console won't turn on! They ask if you can take a look.

// You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.

// The boot code is represented as a text file with one instruction per line of text. Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).

// acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
// jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
// nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
// For example, consider the following program:

// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
// These instructions are visited in this order:

// nop +0  | 1
// acc +1  | 2, 8(!)
// jmp +4  | 3
// acc +3  | 6
// jmp -3  | 7
// acc -99 |
// acc +1  | 4
// jmp -4  | 5
// acc +6  |
// First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the bottom. After it increases the accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.

// This is an infinite loop: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.

// Immediately before the program would run an instruction a second time, the value in the accumulator is 5.

// Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?

// --- Part Two ---
// After some careful analysis, you believe that exactly one instruction is corrupted.

// Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp. (No acc instructions were harmed in the corruption of this boot code.)

// The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.

// For example, consider the same program from above:

// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
// If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction infinite loop, never leaving that instruction. If you change almost any of the jmp instructions, the program will still eventually find another jmp instruction and loop forever.

// However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program terminates! The instructions are visited in this order:

// nop +0  | 1
// acc +1  | 2
// jmp +4  | 3
// acc +3  |
// jmp -3  |
// acc -99 |
// acc +1  | 4
// nop -4  | 5
// acc +6  | 6
// After the last instruction (acc +6), the program terminates by attempting to run the instruction below the last instruction in the file. With this change, after the program terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).

// Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?

use std::clone;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError is here! {}", self.message)
    }
}

impl Error for ParseError {}

#[derive(Clone, Debug, Default)]
struct Bootloader {
    pub program: Program,
    pub acc: i32,
    pub executed_inst_indices: Vec<u32>,
    pub halted: bool,
    program_counter: i32,
}

impl Bootloader {
    fn new() -> Self {
        Bootloader::default()
    }

    fn load_program(&mut self, path: &str) -> Result<&mut Self, Box<dyn Error>> {
        self.program = Program::parse(fs::read_to_string(path)?)?;

        Ok(self)
    }

    fn step(&mut self) -> &mut Self {
        if self
            .executed_inst_indices
            .contains(&(self.program_counter as u32))
        {
            self.halted = true;
            return self;
        }

        let inst: &Inst = &self.program.instructions[self.program_counter as usize];

        let mut inst_offset = 1;

        match inst {
            Inst::Nop(_) => (),
            Inst::Acc(i) => self.acc = self.acc + i,
            Inst::Jmp(i) => inst_offset = *i,
        }

        self.executed_inst_indices.push(self.program_counter as u32);
        self.program_counter = self.program_counter + inst_offset;

        self
    }

    fn reset(&mut self) -> &mut Self {
        self.acc = 0;
        self.program_counter = 0;
        self.halted = false;
        self
    }

    fn completed(&self) -> bool {
        self.program_counter == self.program.instructions.len().try_into().unwrap()
    }
}

#[derive(Debug, Default)]
struct Program {
    instructions: Vec<Inst>,
}

impl Program {
    fn parse(text: String) -> Result<Self, Box<dyn Error>> {
        let mut instructions = Vec::new();

        for line in text.trim_end_matches("\n").split("\n") {
            instructions.push(Program::parse_instruction(line)?)
        }

        Ok(Program { instructions })
    }

    fn parse_instruction(line: &str) -> Result<Inst, Box<dyn Error>> {
        Ok(Inst::from_str(line)?)
    }

    fn fix_inst(&mut self, i: usize) -> &mut Self {
        match self.instructions[i] {
            Inst::Nop(a) => {
                println!("Replacing Nop({}) with Jmp at index: {}", a, i);
                self.instructions[i] = Inst::Jmp(a);
            }
            Inst::Jmp(a) => {
                println!("Replacing Jmp({}) with Nop at index: {}", a, i);
                self.instructions[i] = Inst::Nop(a);
            }
            _ => unreachable!("Invalid index passed to fix_inst/2"),
        }

        self
    }
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program {
            instructions: self.instructions.clone(),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Inst {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Inst {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ");

        match (iter.next(), iter.next()) {
            (Some(inst), Some(value)) => {
                let value = value.trim_start_matches("+").parse()?;

                match inst {
                    "nop" => Ok(Inst::Nop(value)),
                    "acc" => Ok(Inst::Acc(value)),
                    "jmp" => Ok(Inst::Jmp(value)),
                    _ => Err(ParseError {
                        message: String::from("Unknown instruction!"),
                    }
                    .into()),
                }
            }
            _ => Err(ParseError {
                message: String::from("Failed to parse instruction!"),
            }
            .into()),
        }
    }
}

fn part2() {
    let mut bl = Bootloader::new();

    bl.load_program("input.txt").unwrap_or_else(|e| {
        println!("Failed to parse program text! {:?}", e);
        process::exit(1)
    });

    for i in 0..bl.program.instructions.len() {
        // skip Acc instructions, they can't be fixed!
        if let Inst::Acc(_) = bl.program.instructions[i] {
            continue;
        }

        // make a mutable copy of the Bootloader so we can step through the program and accumulate
        let mut fixed_bl = bl.clone();

        // fix the current instruction
        fixed_bl.program = fixed_bl.program.fix_inst(i).to_owned();

        // run to completion or halt
        while !(fixed_bl.completed() || fixed_bl.halted) {
            fixed_bl.step();
        }

        // if completed, this instruction's replacement was the correct one
        if fixed_bl.completed() {
            println!("{:#?}", fixed_bl.acc);
            return;
        }
    }
}

fn part1() {
    let mut bl = Bootloader::new();

    bl.load_program("input.txt").unwrap_or_else(|e| {
        println!("Failed to parse program text! {:?}", e);
        process::exit(1)
    });

    while !bl.halted {
        bl.step();
    }

    println!("{:#?}", bl.acc);
}

fn main() {
    part2()
}
