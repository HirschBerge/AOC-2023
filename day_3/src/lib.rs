use aochelpers::get_daily_input;
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
pub mod part1;
pub mod part2;
use std::error::Error;
use std::process::exit;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    Mul((u32, u32)),
}

impl Instruction {
    fn multiply(&self) -> u32 {
        match self {
            Instruction::Mul((x, y)) => x * y,
        }
    }
}

pub fn gather_data() -> String {
    match get_input() {
        Ok(ans) => ans,
        Err(e) => {
            eprintln!("Failed to get input for today with: {}", e);
            exit(1)
        }
    }
}

// INFO: Just grabs the data from AOC
pub fn get_input() -> Result<String, Box<dyn Error>> {
    let data = get_daily_input(3, 2024)?;
    Ok(data)
}

// NOTE: Handling input data

pub fn parse_instruction(memory: &str) -> IResult<&str, Instruction> {
    let (memory, _) = tag("mul")(memory)?;
    let (memory, nums) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(memory)?;
    Ok((memory, Instruction::Mul(nums)))
}

pub fn mul(prog_mem: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_instruction).map(|(_, ins)| ins))(prog_mem)
}
