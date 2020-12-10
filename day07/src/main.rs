use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

use nom::{
    IResult,
    branch::alt,

    multi::separated_list1,

    sequence::separated_pair,
    sequence::terminated,

    character::complete::alpha1,
    character::complete::digit1,

    combinator::map_res,

    bytes::complete::tag,
};

#[derive(Debug)]
struct Rule {
    color: String,
    holds: Vec<(usize, String)>
}

type IRule = Vec<(usize, String)>;

#[derive(Debug)]
struct Rules {
    map: HashMap<String, IRule>,
    nmap: HashMap<String, IRule>
}

impl Rules {
    fn new() -> Self {
        Self { map: HashMap::new(), nmap: HashMap::new() }
    }

    fn add_rule(&mut self, rule: Rule) {
        for (num, color) in rule.holds.iter() {
            match self.map.get_mut(color) {
                None => {
                    self.map.insert(
                        color.to_string(),
                        vec![(*num, rule.color.to_string())]
                    );
                },
                Some(vec) => vec.push((*num, rule.color.to_string()))
            }
        }

        self.nmap.insert(rule.color.to_string(), rule.holds);
    }

    fn find_deps(&self, color: &str) -> usize {
        let mut acc = 0;
        let mut done = HashSet::<String>::new();
        let mut doing = self.map.get(color).unwrap().clone();

        while doing.len() != 0 {
            let mut next = Vec::<(usize, String)>::new();

            for (_, color) in doing.iter() {
                if done.insert(color.to_string()) {
                    acc += 1;
                    for color in self.map.get(color).unwrap_or(&Vec::new()).iter() {
                        next.push(color.clone());
                    };
                }
            }

            doing = next;
        }

        acc
    }

    fn count(&self, color: &str) -> usize {
        match self.nmap.get(color) {
            None => 0,
            Some(held) => held.into_iter()
                    .fold(0, |acc, (num, color)| acc + num + num * self.count(color))
        }
    }
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |a: &str| a.parse())(input)
}

fn parse_color(input: &str) -> IResult<&str, String> {
    let (input, (color1, color2)) = separated_pair(alpha1, tag(" "), alpha1)(input)?;
    Ok((input, format!("{} {}", color1, color2)))
}

fn parse_held(input: &str) -> IResult<&str, (usize, String)> {
    separated_pair(number, tag(" "), terminated(parse_color, terminated(tag(" "), alpha1)))(input)
}

fn parse_empty(input: &str) -> IResult<&str, Vec<(usize, String)>> {
    let (input, _) = tag("no other bags")(input)?;
    Ok((input, vec![]))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (color, holds)) = terminated(
        separated_pair(
            parse_color, tag(" bags contain "),
            alt((
                separated_list1(tag(", "), parse_held),
                parse_empty))),
        tag("."))(input)?;

    Ok((input, Rule { color, holds }))
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file); 
    let mut rules = Rules::new();

    for line in reader.lines() {
        let (_, rule) = parse_rule(&line?).unwrap();
        rules.add_rule(rule);
    }

    let answer1 = rules.find_deps("shiny gold");
    let answer2 = rules.count("shiny gold");

    println!("(1) There are {} ways to have a shiny gold bag!", answer1);
    println!("(2) You need {} bags inside your shiny gold bag!", answer2);

    Ok(())
}
