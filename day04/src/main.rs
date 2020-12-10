use nom::{
    bytes::complete::tag, character::complete::digit1, bytes::complete::take_while,
    multi::separated_list1, bytes::complete::take_while_m_n,
    combinator::map_res, combinator::map, sequence::preceded, IResult, sequence::tuple,
    branch::alt,
};
use std::fs::read_to_string;
use std::io;
use std::vec::Vec;

#[derive(Debug)]
enum PassportField<'a> {
    BirthYear(usize),
    IssueYear(usize),
    ExpirationYear(usize),
    HairColor(&'a str),
    EyeColor(&'a str),
    PassportId(&'a str),
    CountryId(&'a str),
    Height(&'a str),
}

use PassportField::*;

type Passport<'a> = Vec<PassportField<'a>>;

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |a: &str| a.parse())(input)
}

fn is_val(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= '0' && c <= '9' || c >= 'A' && c <= 'Z' || c == '#'
}

fn parse_word(input: &str) -> IResult<&str, &str> {
    take_while(is_val)(input)
}

fn parse_field(input: &str) -> IResult<&str, PassportField> {
    alt((
        map(preceded(tag("byr:"), parse_number), BirthYear),
        map(preceded(tag("iyr:"), parse_number), IssueYear),
        map(preceded(tag("eyr:"), parse_number), ExpirationYear),
        map(preceded(tag("hcl:"), parse_word), HairColor),
        map(preceded(tag("ecl:"), parse_word), EyeColor),
        map(preceded(tag("pid:"), parse_word), PassportId),
        map(preceded(tag("cid:"), parse_word), CountryId),
        map(preceded(tag("hgt:"), parse_word), Height),
    ))(input)
}

fn parse_whitespace(input: &str) -> IResult<&str, &str> {
    alt((tag(" "), tag("\t"), tag("\n"), tag("\r\n")))(input)
}

fn parse_fields(input: &str) -> IResult<&str, Passport> {
    separated_list1(parse_whitespace, parse_field)(input)
}

fn parse_passports(input: &str) -> IResult<&str, Vec<Passport>> {
    separated_list1(tag("\n\n"), parse_fields)(input)
}

fn is_valid1(passport: &Passport) -> bool {
    let mut found_byr = false;
    let mut found_iyr = false;
    let mut found_eyr = false;
    let mut found_hcl = false;
    let mut found_ecl = false;
    let mut found_pid = false;
    let mut found_hgt = false;

    for field in passport.iter() {
        match field {
            BirthYear(_) => found_byr = true,
            IssueYear(_) => found_iyr = true,
            ExpirationYear(_) => found_eyr = true,
            HairColor(_) => found_hcl = true,
            EyeColor(_) => found_ecl = true,
            PassportId(_) => found_pid = true,
            Height(_) => found_hgt = true,
            _ => {},
        }
    }

    return found_hgt && found_pid && found_ecl && found_hcl && found_eyr && found_iyr && found_byr;
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn parse_hcl(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), take_while_m_n(6, 6, is_hex_digit))(input)
}

fn is_hcl(c: &str) -> bool {
    if let Ok(("", _)) = parse_hcl(c) {
        true
    } else {
        false
    }
}

fn parse_pid(input: &str) -> IResult<&str, &str> {
    take_while_m_n(9, 9, |c: char| c.is_digit(10))(input)
}

fn is_pid(c: &str) -> bool {
    if let Ok(("", _)) = parse_pid(c) {
        true
    } else {
        false
    }
}

fn parse_hgt(input: &str) -> IResult<&str, bool> {
    let (input, (num, unit)) = tuple((parse_number, alt((tag("in"), tag("cm")))))(input)?;

    if unit == "in" {
        Ok((input, num >= 59 && num <= 76))
    } else {
        Ok((input, num >= 150 && num <= 193))
    }
}

fn is_hgt(c: &str) -> bool {
    if let Ok(("", r)) = parse_hgt(c) {
        r
    } else {
        false
    }
}

fn is_valid2(passport: &Passport) -> bool {
    let mut found_byr = false;
    let mut found_iyr = false;
    let mut found_eyr = false;
    let mut found_hcl = false;
    let mut found_ecl = false;
    let mut found_pid = false;
    let mut found_hgt = false;

    for field in passport.iter() {
        match field {
            BirthYear(y) => found_byr = y >= &1920 && y <= &2002,
            IssueYear(y) => found_iyr = y >= &2010 && y <= &2020,
            ExpirationYear(y) => found_eyr = y >= &2020 && y <= &2030,
            HairColor(c) => found_hcl = is_hcl(c),
            EyeColor(c) => found_ecl = match c.as_ref() {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false,
            },
            PassportId(i) => found_pid = is_pid(i),
            Height(h) => found_hgt = is_hgt(h),
            _ => {},
        }
    }

    return found_hgt && found_pid && found_ecl && found_hcl && found_eyr && found_iyr && found_byr;
}

fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?;
    let (_, passports) = parse_passports(&input).unwrap();

    let mut valid1 = 0;
    let mut valid2 = 0;

    for passport in passports.iter() {
        if is_valid1(&passport) {
            valid1 += 1;
        }

        if is_valid2(&passport) {
            valid2 += 1;
        }
    }

    println!("(1) There are {} valid passports!", valid1);
    println!("(2) There are {} valid passports!", valid2);

    Ok(())
}
