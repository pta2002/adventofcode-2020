use nom::{
    bytes::complete::tag, bytes::complete::take_until, character::complete::digit1,
    character::complete::line_ending, combinator::map_res, multi::many1, multi::separated_list1,
    sequence::preceded, sequence::separated_pair, sequence::terminated, IResult,
};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;
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
        (num <= self.range1.end && num >= self.range1.start)
            || (num <= self.range2.end && num >= self.range2.start)
    }
}

#[derive(Debug)]
struct Notes {
    rules: Vec<Rule>,
    yours: Ticket,
    nearby: Vec<Ticket>,
    fields: HashMap<usize, String>,
}

impl Notes {
    fn check_tickets(&self) -> usize {
        // This is fairly easy
        // All we have to do is iterate over each ticket. Once there, we iterate over every field.
        // Then, over every rule. If none of the rules are valid, we add the field to the return
        // value. If at least one is, we short-circuit and just skip ahead! Shouldn't be terribly
        // hard to do any of this.
        self.nearby.iter().fold(0, |s, t| s + self.check_ticket(t).iter().fold(0, |s, t| s + t))
    }

    fn check_ticket(&self, ticket: &Ticket) -> Vec<usize> {
        let mut count = vec![];
        'field: for field in ticket {
            for rule in &self.rules {
                if rule.check_value(*field) {
                    continue 'field;
                }
            }
            count.push(*field);
        }

        count
    }

    fn filter_bad_tickets(&mut self) {
        let mut i = 0;
        while i < self.nearby.len() {
            if self.check_ticket(&self.nearby[i]).len() != 0 {
                self.nearby.remove(i);
                continue;
            }
            i += 1;
        }
    }

    fn match_fields(&self) -> Vec<String> {
        let mut possible_fields = vec![];

        for i in 0..self.rules.len() {
            possible_fields.push(self.find_fields(i));
        }

        let mut done: Vec<usize> = vec![];

        while done.len() != possible_fields.len() {
            let mut i = 0;

            while i < possible_fields.len() {
                if !done.iter().any(|&j| j == i) && possible_fields[i].len() == 1 {
                    done.push(i);
                    let needle = possible_fields[i][0];
                    
                    for field in &mut possible_fields {
                        if field.len() != 1 {
                            if let Some(pos) = field.iter().position(|&x| x == needle) {
                                field.remove(pos);
                            }
                        }
                    }
                }

                i += 1;
            }
        }

        possible_fields.iter().map(|a| self.rules[a[0]].name.clone()).collect()
    }

    fn my_fields(&self) -> usize {
        let matches = self.match_fields();
        let mut ret = 1;

        for (i, field) in matches.iter().enumerate() {
            if field.starts_with("departure") {
                ret *= self.yours[i];
            }
        }

        ret
    }

    fn find_fields(&self, col: usize) -> Vec<usize> {
        let mut ret: Vec<usize> = self.rules.iter().enumerate().map(|(i, _)| i).collect();

        for ticket in &self.nearby {
            let value = ticket[col];

            let mut i = 0;
            while i < ret.len() {
                if !self.rules[ret[i]].check_value(value) {
                    ret.remove(i);
                    continue;
                } 

                i += 1;
            }
        }

        ret
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
            fields: HashMap::new(),
        },
    ))
}

fn main() -> Result<(), io::Error> {
    let file = read_to_string("input.txt")?;

    let (_, mut input) = notes(&file).unwrap();

    println!(
        "(1) The ticket scanning error rate is {}",
        input.check_tickets()
    );

    input.filter_bad_tickets();

    println!("(2) The answer is {}", input.my_fields());

    Ok(())
}
