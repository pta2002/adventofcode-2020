use std::fs::File;
use std::io::{self, BufReader, BufRead};
use nom::{
    IResult,

    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,

    combinator::map,
    combinator::map_res,
    sequence::preceded,
};

#[derive(Debug, Clone, Copy)]
enum Route {
    Forward(i32),
    Right(i32),
    Left(i32),
    North(i32),
    South(i32),
    West(i32),
    East(i32),
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
    facing: i32,
}

impl Pos {
    fn forward(&self, i: i32) -> Self {
        if self.facing == 0 {
            Pos { x: self.x + i, y: self.y, facing: self.facing }
        } else if self.facing == 90 {
            Pos { x: self.x, y: self.y + i, facing: self.facing }
        } else if self.facing == 180 {
            Pos { x: self.x - i, y: self.y, facing: self.facing }
        } else {
            Pos { x: self.x, y: self.y - i, facing: self.facing }
        }
    }

    fn rotate(&self, i: i32) -> Self {
        // It took me so fucking long to figure this one out!
        // Pad it by 360 degrees so that negative numbers wrap properly
        Pos { x: self.x, y: self.y, facing: (self.facing + 360 + i) % 360 }
    }
}

#[derive(Debug, Clone)]
struct Waypoint {
    ship_x: i32,
    ship_y: i32,
    x: i32,
    y: i32,
}

impl Waypoint {
    fn forward(&self, mut i: i32) -> Self {
        let mut ret = self.clone();
        while i > 0 {
            i -= 1;
            ret.ship_x += self.x;
            ret.ship_y += self.y;
        }

        ret
    }

    fn new() -> Self {
        Self { x: 10, y: -1, ship_x: 0, ship_y: 0 }
    }

    fn rotate(&self, i: i32) -> Self {
        if i == 0 {
            self.clone()
        } else {
            (Self { ship_x: self.ship_x, ship_y: self.ship_y, x: -self.y, y: self.x }).rotate(i - 90)
        }
    }
}

fn step1(pos: Pos, route: Route) -> Pos {
    match route {
        Route::Forward(i) => pos.forward(i),
        Route::Right(i) => pos.rotate(i),
        Route::Left(i) => pos.rotate(-i),
        Route::North(i) => Pos { x: pos.x, y: pos.y - i, facing: pos.facing },
        Route::South(i) => Pos { x: pos.x, y: pos.y + i, facing: pos.facing },
        Route::West(i)  => Pos { x: pos.x - i, y: pos.y, facing: pos.facing },
        Route::East(i)  => Pos { x: pos.x + i, y: pos.y, facing: pos.facing },
    }
}

fn step2(waypoint: Waypoint, route: Route) -> Waypoint {
    match route {
        Route::Forward(i) => waypoint.forward(i),
        Route::Right(i) => waypoint.rotate(i),
        Route::Left(i) => waypoint.rotate(360 - i),
        Route::North(i) => Waypoint { x: waypoint.x, y: waypoint.y - i, ship_x: waypoint.ship_x, ship_y: waypoint.ship_y },
        Route::South(i) => Waypoint { x: waypoint.x, y: waypoint.y + i, ship_x: waypoint.ship_x, ship_y: waypoint.ship_y },
        Route::West(i)  => Waypoint { x: waypoint.x - i, y: waypoint.y, ship_x: waypoint.ship_x, ship_y: waypoint.ship_y },
        Route::East(i)  => Waypoint { x: waypoint.x + i, y: waypoint.y, ship_x: waypoint.ship_x, ship_y: waypoint.ship_y },
    }
}

fn parse_number<'a>(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |a: &str| a.parse())(input)
}

fn parse_route(input: &str) -> Route {
    alt((
        map(preceded(tag("F"), parse_number), Route::Forward),
        map(preceded(tag("R"), parse_number), Route::Right),
        map(preceded(tag("L"), parse_number), Route::Left),
        map(preceded(tag("N"), parse_number), Route::North),
        map(preceded(tag("W"), parse_number), Route::West),
        map(preceded(tag("S"), parse_number), Route::South),
        map(preceded(tag("E"), parse_number), Route::East)))(input).unwrap().1
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let (answer1, answer2) = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|a| parse_route(&a))
        .fold((Pos { x: 0, y: 0, facing: 0 }, Waypoint::new()), |(a, b), r| (step1(a,r), step2(b, r)));

    println!("(1) Distance is {}", answer1.x.abs() + answer1.y.abs());
    println!("(2) Distance is {}", answer2.ship_x.abs() + answer2.ship_y.abs());

    Ok(())
}
