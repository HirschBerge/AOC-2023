use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
pub mod part1;
pub mod part2;

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
pub fn get_daily_input(year: i32, day: i32)  -> String {
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
