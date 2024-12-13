use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{self, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use glam::IVec2;
pub mod part1;
pub mod part2;
use std::str::FromStr;

pub fn get_daily_input(year: usize, day: usize) -> String {
    let path = format!("../.inputs/{}/{}", year, day);
    match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            // Handle the error here, e.g., return an error code or exit the program
            std::process::exit(1);
        }
    }
}

// NOTE: Handling input data

pub fn _nom_into_nums(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(
        line_ending,
        separated_pair(
            map_res(take_while1(|c: char| c.is_ascii_digit()), usize::from_str),
            space1,
            map_res(take_while1(|c: char| c.is_ascii_digit()), usize::from_str),
        ),
    )(input)
}

#[derive(Debug)]
struct Machine {
    a: IVec2,
    b: IVec2,
    prize: IVec2,
}

fn a_button(input: &str) -> IResult<&str, IVec2> {
    nom::sequence::preceded(
        tag("Button A: X+"),
        separated_pair(complete::i32, tag(", Y+"), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)
}
fn b_button(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("Button B: X+"),
        separated_pair(complete::i32, tag(", Y+"), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)
}
fn prize(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("Prize: X="),
        separated_pair(complete::i32, tag(", Y="), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a, b, p)) = tuple((
        terminated(a_button, line_ending),
        terminated(b_button, line_ending),
        prize,
    ))(input)?;

    Ok((input, Machine { a, b, prize: p }))
}
fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(tuple((line_ending, line_ending)), machine)(input)
}
