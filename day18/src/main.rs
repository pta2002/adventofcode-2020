use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    combinator::map, combinator::map, combinator::map, combinator::map, combinator::map_res,
    combinator::value, IResult,
};
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Multiply,
    Add,
}

#[derive(Debug)]
enum Expression<'a> {
    Value(usize),
    Expression(&'a Expression<'a>, Operator, &'a Expression<'a>),
}

impl<'a> Expression<'a> {
    fn eval(&self) -> usize {
        match self {
            Expression::Value(i) => *i,
            Expression::Expression(a, o, b) => match o {
                Operator::Add => a.eval() + b.eval(),
                Operator::Multiply => a.eval() * b.eval(),
            },
        }
    }
}

fn number(input: &str) -> IResult<&str, Expression> {
    map(map_res(digit1, |a: &str| a.parse()), Expression::Value)(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Add, tag("+")),
        value(Operator::Multiply, tag("*")),
    ))(input)
}

fn expression(input: &str) -> IResult<&str, Expression> {
    // 1st value - Either single number, or a parenthesis-wrapped expression. Try the expression
    // FIRST
    // 2nd value - Only an expression
    todo!()
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    Ok(())
}
