use std::fs::read_to_string;
use std::io;
use nom::{
    IResult,

    bytes::complete::tag,

    character::complete::line_ending,
    character::complete::digit1,

    combinator::map_res,
    combinator::map,
    combinator::value,

    multi::separated_list1,

    sequence::separated_pair,

    branch::alt
};

#[derive(Debug, Clone)]
enum Bus {
    X,
    Time(usize)
}

#[derive(Debug)]
struct Timetable {
    time: usize,
    busses: Vec<Bus>
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |a: &str| a.parse())(input)
}

fn parse_busses(input: &str) -> IResult<&str, Vec<Bus>> {
    separated_list1(tag(","), alt((value(Bus::X, tag("x")), map(parse_number, Bus::Time))))(input)
}

fn parse_timetable(input: &str) -> Option<Timetable> {
    let (_, (time, busses)) = separated_pair(parse_number, line_ending, parse_busses)(input).ok()?;
    
    Some(Timetable { time, busses })
}

fn main() -> Result<(), io::Error> {
    let timetable = parse_timetable(&read_to_string("input.txt")?).unwrap();

    let mut min = None;
    let mut id = None;

    for i in &timetable.busses {
        if let Bus::Time(x) = i {
            let q: usize = (timetable.time / x) * x + x;

            match min {
                None => {
                    min = Some(q);
                    id = Some(x)
                },
                Some(i) => if q < i {
                    min = Some(q);
                    id = Some(x);
                }
            }
        }
    }

    let answer1 = (min.unwrap() - timetable.time) * id.unwrap();

    println!("(1) Answer is {}", answer1);

    // Some observations for part 2:
    // The numbers are all prime. We can find the minimum common multiple by just multiplying them
    // together.
    // I need to find a solution to:
    // 7*a = 13*b - 1 = 59*c - 4 = 31*d - 6 = 19*e - 7
    // So let's do it the... shameful... way - offload it to wolfram alpha!

    println!("(2) Put this into wolfram alpha, and grab the integer solution for a, then multiply it by a's coefficient!");

    let len = timetable.busses.len();
    let mut lastletter = 'a';
    for i in 0..len {
        if let Bus::Time(x) = timetable.busses[i] {
            if i == 0 {
                print!("{}{}", x, lastletter);
            } else {
                print!(" = {}{} - {}", x, lastletter, i);
            }

            lastletter = std::char::from_u32(lastletter as u32 + 1).unwrap_or(lastletter);
        }
    }

    println!("");
    println!("(Sorry about that)");

    Ok(())
}
