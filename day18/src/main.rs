use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    combinator::map_res, combinator::value, multi::fold_many0, sequence::pair, sequence::preceded,
    sequence::terminated, sequence::tuple, IResult,
};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Multiply,
    Add,
}

#[derive(Debug, Clone)]
enum Expression {
    Value(usize),
    Expression(Box<Expression>, Operator, Box<Expression>),
}

impl Expression {
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

fn single_expr1(input: &str) -> IResult<&str, Expression> {
    let paren_expr = preceded(tag("("), terminated(expression1, tag(")")));
    alt((paren_expr, number))(input)
}

fn single_expr2(input: &str) -> IResult<&str, Expression> {
    let paren_expr = preceded(tag("("), terminated(expression2, tag(")")));
    alt((paren_expr, number))(input)
}

fn expression1(mut input: &str) -> IResult<&str, Expression> {
    let (ninput, mut expr) = single_expr1(input)?;
    input = ninput;

    loop {
        match tuple((
            preceded(tag(" "), terminated(operator, tag(" "))),
            single_expr1,
        ))(input)
        {
            Err(_) => break,
            Ok((ninput, (operator, nexpr))) => {
                input = ninput;
                expr = Expression::Expression(Box::new(expr), operator, Box::new(nexpr));
            }
        }
    }

    Ok((input, expr))
}

fn add(input: &str) -> IResult<&str, Expression> {
    let (i, init) = single_expr2(input)?;

    fold_many0(pair(tag(" + "), single_expr2), init, |acc, (_, val)| {
        Expression::Expression(Box::new(acc), Operator::Add, Box::new(val))
    })(i)
}

fn expression2(input: &str) -> IResult<&str, Expression> {
    let (i, init) = add(input)?;

    fold_many0(pair(tag(" * "), add), init, |acc, (_, val)| {
        Expression::Expression(Box::new(acc), Operator::Multiply, Box::new(val))
    })(i)
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in reader.lines() {
        let line = line?;
        let (_, expr1) = expression1(&line).unwrap();
        sum1 += expr1.eval();

        let (_, expr2) = expression2(&line).unwrap();
        sum2 += expr2.eval();
    }

    println!("(1) Sum is {}", sum1);
    println!("(2) Sum is {}", sum2);

    Ok(())
}
