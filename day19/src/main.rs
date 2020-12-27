use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    combinator::map_res, combinator::value, sequence::delimited, IResult,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::rc::Rc;

#[derive(Debug, Clone)]
enum Rule {
    Or(Box<Rule>, Box<Rule>),
    Char(char),
    // TODO this should be a vec!
    Sequence(Vec<usize>),
}

type Rules = HashMap<usize, Rc<Rule>>;

// This is a nice little parser combinator!
impl Rule {
    // If the parser fails, it returns None
    // If it succeeds, it returns Some(remaining_chars)
    fn parse<'a>(&self, inputs: &Vec<&'a str>, list: &Rules) -> Vec<&'a str> {
        // Haskell does this SO WELL...
        let mut ret = vec![];
        for input in inputs {
            match self {
                &Rule::Char(i) => {
                    if input.len() > 0 && i == input.chars().nth(0).unwrap() {
                        // Master of overcomplication over here! Works though
                        ret.push(std::str::from_utf8(&input.as_bytes()[1..]).unwrap());
                    }
                }
                // Clean functional code time :)
                Rule::Sequence(a) => {
                    let mut inputs = vec![*input];
                    for rule in a {
                        let rule = list.get(&rule).unwrap();

                        inputs = rule.parse(&inputs, list);
                    }

                    for input in inputs {
                        ret.push(input);
                    }
                }
                Rule::Or(a, b) => {
                    // Well this is essentially cursed! I think the best way to get out of part 2's
                    // mess is to return a list of inputs, and when we parse, we have to check against
                    // each and every one of them...
                    //
                    // Seriously, what a mess.
                    for i in a.parse(&vec![*input], list) {
                        ret.push(i);
                    }

                    for i in b.parse(&vec![*input], list) {
                        ret.push(i);
                    }
                }
            }
        }

        ret
    }

    fn parses(&self, input: &str, list: &Rules) -> bool {
        self.parse(&vec![input], list).contains(&"")
    }
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |a: &str| a.parse())(input)
}

fn one_char(input: &str) -> IResult<&str, Rule> {
    alt((
        value(Rule::Char('a'), tag("a")),
        value(Rule::Char('b'), tag("b")),
    ))(input)
}

fn sequence(input: &str) -> IResult<&str, Rule> {
    map(separated_list1(tag(" "), number), Rule::Sequence)(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    alt((
        delimited(tag("\""), one_char, tag("\"")),
        map(separated_pair(sequence, tag(" | "), sequence), |(a, b)| {
            Rule::Or(Box::new(a), Box::new(b))
        }),
        sequence,
    ))(input)
}

fn numbered_rule(input: &str) -> IResult<&str, (usize, Rule)> {
    separated_pair(number, tag(": "), rule)(input)
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut rules = Rules::new();
    let mut msgs = vec![];

    let mut count1 = 0;
    let mut count2 = 0;

    for line in reader.lines() {
        let line = line?;

        match numbered_rule(&line) {
            Ok((_, (i, rule))) => {
                rules.insert(i, Rc::new(rule));
            }
            _ => {
                if line.len() > 0 {
                    msgs.push(line)
                }
            }
        }
    }

    {
        let rule0 = rules.get(&0).unwrap();

        for msg in &msgs {
            if rule0.parses(msg, &rules) {
                count1 += 1;
            }
        }
    }

    println!("(1) Counted {} valid messages", count1);

    rules.insert(8, Rc::new(rule("42 | 42 8").unwrap().1));
    rules.insert(11, Rc::new(rule("42 31 | 42 11 31").unwrap().1));

    let rule0 = rules.get(&0).unwrap();
    for msg in msgs {
        // I really have no idea why this does not work...
        if rule0.parses(&msg, &rules) {
            count2 += 1;
        }
    }

    println!("(2) Counted {} valid messages", count2);

    Ok(())
}
