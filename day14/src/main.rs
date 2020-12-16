use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};
use std::fmt;

use nom::{
    IResult,

    combinator::map_res,
    combinator::map,

    bytes::complete::tag,

    branch::alt,

    sequence::preceded,
    sequence::terminated,

    character::complete::digit1,
    character::complete::alphanumeric1,
};

#[derive(Clone, Copy)]
enum OneBit {
    Keep,
    One,
    Zero
}

impl fmt::Debug for OneBit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OneBit::Keep => write!(f, "X"),
            OneBit::One => write!(f, "1"),
            OneBit::Zero => write!(f, "0")
        }
    }
}

#[derive(Debug, Clone)]
struct BitMask {
    mask: [OneBit; 36]
}

impl BitMask {
    fn new(mask: &str) -> Self {
        let mut ret = [OneBit::Keep; 36];
        
        for (i, b) in mask.chars().enumerate() {
            match b {
                '1' => ret[i] = OneBit::One,
                '0' => ret[i] = OneBit::Zero,
                _ => {},
            }
        }

        BitMask { mask: ret }
    }

    fn apply(&self, num: u64) -> [OneBit; 36] {
        let mut ret = [OneBit::Zero; 36];

        for (i, b) in self.mask.iter().enumerate() {
            ret[i] = match *b {
                OneBit::Keep => if (num & (1 << (35 - i))) >> (35 - i) == 0 {
                    OneBit::Zero
                } else {
                    OneBit::One
                },
                a => a
            }
        }

        ret
    }

    fn applications(&self, num: u64) -> Vec<[OneBit; 36]> {
        let mut ret = vec![[OneBit::Zero; 36]];

        for (i, b) in self.mask.iter().enumerate().rev() {
            match b {
                OneBit::Keep => {
                    let new_mask_one
                }
            }
        }

        ret
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Mask(BitMask),
    Set(u64, u64)
}

struct Program {
    mask: Option<BitMask>,
    memory: HashMap<u64, u64>,
    ip: usize,
    instructions: Vec<Instruction>
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions, ip: 0, mask: None, memory: HashMap::new() }
    }

    fn step(&mut self) -> bool {
        match &self.instructions[self.ip] {
            Instruction::Mask(mask) => self.mask = Some(mask.clone()),
            Instruction::Set(place, number) => {
                let masked = self.mask.as_ref().unwrap().apply(*number);
                self.memory.insert(*place, from_bits(&masked));
            }
        }

        self.ip += 1;

        self.ip < self.instructions.len()
    }

    fn step2(&mut self) -> bool {
        match &self.instructions[self.ip] {
            Instruction::Mask(mask) => self.mask = Some(mask.clone()),
            Instruction::Set(place, number) => {
                let masked = self.mask.as_ref().unwrap().applications(*number);
                // self.memory.insert(*place, from_bits(&masked));
            }
        }

        self.ip += 1;

        self.ip < self.instructions.len()
    }
}

fn from_bits(bits: &[OneBit; 36]) -> u64 {
    let mut ret = 0;
    for (i, b) in bits.iter().enumerate() {
        let bit = match *b {
            OneBit::Zero => 0,
            OneBit::One => 1,
            _ => panic!("Invalid bit")
        };
        ret = ret | (bit << (35 - i));
    }

    ret
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |a: &str| a.parse())(input)
}

fn parse_mask(input: &str) -> IResult<&str, Instruction> {
    map(map(preceded(tag("mask = "), alphanumeric1), BitMask::new), Instruction::Mask)(input)
}

fn parse_mem(input: &str) -> IResult<&str, Instruction> {
    let (input, location) = terminated(preceded(tag("mem["), parse_number), tag("] = "))(input)?;
    let (input, value) = parse_number(input)?;

    Ok((input, Instruction::Set(location, value)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mask, parse_mem))(input)
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut instructions = vec![];

    for l in reader.lines() {
        let (_, ins) = parse_instruction(&l?).unwrap();
        instructions.push(ins);
    }

    let instructions2 = instructions.clone();

    let mut program = Program::new(instructions);

    while program.step() {}

    let answer1 = program.memory.iter()
        .map(|(_,v)| v)
        .fold(0, |acc, val| acc + val);

    let mut program2 = Program::new(instructions2);
    while program2.step2() {}

    let answer2 = program.memory.iter()
        .map(|(_,v)| v)
        .fold(0, |acc, val| acc + val);

    println!("(1) Sum of values in memory is {}", answer1);
    println!("(1) Sum of values in memory is {}", answer2);

    Ok(())
}
