use nom::{
    bytes::complete::tag, character::complete::anychar, character::complete::digit1,
    combinator::map_res, sequence::separated_pair, sequence::terminated, IResult,
};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn parse_digit<'a>(input: &'a str) -> IResult<&'a str, usize> {
    map_res(digit1, |a: &str| a.parse())(input)
}

fn parse_min_max<'a>(input: &'a str) -> IResult<&'a str, (usize, usize)> {
    separated_pair(parse_digit, tag("-"), parse_digit)(input)
}

fn parse_rule(input: &str) -> IResult<&str, ((usize, usize), char)> {
    separated_pair(parse_min_max, tag(" "), anychar)(input)
}

fn parse_password(input: &str) -> IResult<&str, Password> {
    let (input, ((min, max), letter)) = terminated(parse_rule, tag(": "))(input)?;

    Ok((
        "",
        Password {
            min,
            max,
            letter,
            password: input.to_string(),
        },
    ))
}

fn verify_password1(password: &Password) -> bool {
    let mut occurrences = 0;

    for letter in password.password.chars() {
        if letter == password.letter {
            occurrences += 1;
        }
    }

    occurrences >= password.min && occurrences <= password.max
}

fn verify_password2(password: &Password) -> bool {
    let letter1 = password.password.chars().nth(password.min - 1).unwrap();
    let letter2 = password.password.chars().nth(password.max - 1).unwrap();

    (letter1 == password.letter && letter2 != password.letter)
        || (letter1 != password.letter && letter2 == password.letter)
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut valid1 = 0;
    let mut valid2 = 0;

    for line in reader.lines() {
        let line = line?;
        let (_, password) = parse_password(&line).unwrap();
        if verify_password1(&password) {
            valid1 += 1;
        }

        if verify_password2(&password) {
            valid2 += 1;
        }
    }

    println!("(1) {} valid passwords!", valid1);
    println!("(2) {} valid passwords!", valid2);

    Ok(())
}
