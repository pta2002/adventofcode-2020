use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::convert::TryInto;

use nom::{
    IResult,
    branch::alt,

    sequence::preceded,

    combinator::map,
    combinator::map_res,

    character::complete::digit1,

    bytes::complete::tag,
};

#[derive(Debug)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32)
}

#[derive(Debug)]
struct Program {
    acc: i32,
    acc_before: i32,
    ip: usize,
    instructions: Vec<(usize, Instruction)>,
}

impl Program {
    fn new() -> Self {
        Self { acc: 0, acc_before: 0, ip: 0, instructions: vec![] }
    }

    fn step(&mut self) -> Option<usize> {
        if self.instructions.len() <= self.ip {
            None
        } else {
            self.instructions[self.ip].0 += 1;
            let (ret, ins) = &self.instructions[self.ip];

            match ins {
                Instruction::Acc(val) => {
                    self.acc_before = self.acc;
                    self.acc += val
                },
                Instruction::Nop(_) => {},
                Instruction::Jmp(val) => {
                    let ip32: i32 = self.ip.try_into().unwrap();
                    let newip = ip32 + val - 1;
                    self.ip = newip.try_into().unwrap()
                }
            }

            self.ip += 1;

            Some(*ret)
        }
    }
}

fn parse_num(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |a: &str| a.parse())(input)
}

fn parse_signed(input: &str) -> IResult<&str, i32> {
    alt((
        map(preceded(tag("-"), parse_num), |n| -n),
        preceded(tag("+"), parse_num)))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
            map(preceded(tag("nop "), parse_signed), Instruction::Nop),
            map(preceded(tag("acc "), parse_signed), Instruction::Acc),
            map(preceded(tag("jmp "), parse_signed), Instruction::Jmp)
        ))(input)
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file); 

    let mut program = Program::new();

    for line in reader.lines() {
        let (_, ins) = parse_instruction(&line?).unwrap();
        program.instructions.push((0, ins));
    }

    loop {
        let step = program.step();
        if let Some(2) = step {
            println!("(1) Accumulator is {}", program.acc_before);
            break;
        }

        if step == None {
            break;
        }
    }

    Ok(())
}
