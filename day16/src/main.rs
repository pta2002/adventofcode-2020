use nom::{
    bytes::complete::tag, bytes::complete::take_until, character::complete::digit1,
    character::complete::line_ending, combinator::map_res, multi::many1, multi::separated_list1,
    sequence::preceded, sequence::separated_pair, sequence::terminated, IResult,
};
use std::fs::read_to_string;
use std::io::{self, BufRead, BufReader};
use std::ops::Range;

type Ticket = Vec<usize>;

#[derive(Debug)]
struct Rule {
    name: String,
    range1: Range<usize>,
    range2: Range<usize>,
}

impl Rule {
    fn check_value(&self, num: usize) -> bool {
        (num <= self.range1.end && num >= self.range1.start) || (num >= self.range2.end && num <= self.range2.start)
    }
}

#[derive(Debug)]
struct Notes {
    rules: Vec<Rule>,
    yours: Ticket,
    nearby: Vec<Ticket>,
}

impl Notes {
    fn check_tickets(&self) -> usize {
        // This is fairly easy
        // All we have to do is iterate over each ticket. Once there, we iterate over every field.
        // Then, over every rule. If none of the rules are valid, we add the field to the return
        // value. If at least one is, we short-circuit and just skip ahead! Shouldn't be terribly
        // hard to do any of this.
        todo!()
    }
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |a: &str| a.parse())(input)
}

fn range(input: &str) -> IResult<&str, Range<usize>> {
    let (input, (a, b)) = separated_pair(number, tag("-"), number)(input)?;

    Ok((input, a..b))
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = take_until(": ")(input)?;
    let (input, (range1, range2)) =
        preceded(tag(": "), separated_pair(range, tag(" or "), range))(input)?;

    Ok((
        input,
        Rule {
            name: String::from(name),
            range1,
            range2,
        },
    ))
}

fn ticket(input: &str) -> IResult<&str, Ticket> {
    separated_list1(tag(","), number)(input)
}

fn notes(input: &str) -> IResult<&str, Notes> {
    let (input, rules) = separated_list1(line_ending, rule)(input)?;

    let (input, yours) = preceded(
        preceded(
            many1(line_ending),
            terminated(tag("your ticket:"), line_ending),
        ),
        ticket,
    )(input)?;

    let (input, nearby) = preceded(
        preceded(
            many1(line_ending),
            terminated(tag("nearby tickets:"), line_ending),
        ),
        separated_list1(line_ending, ticket),
    )(input)?;

    Ok((
        input,
        Notes {
            rules,
            nearby,
            yours,
        },
    ))
}

fn main() -> Result<(), io::Error> {
    let file = read_to_string("input.txt")?;

    let (_, input) = notes(&file).unwrap();

    println!("(1) The ticket scanning error rate is {}", input.check_tickets());

    Ok(())
}
