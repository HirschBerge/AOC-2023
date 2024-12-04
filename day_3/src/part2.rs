use crate::get_daily_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    Mul((u32, u32)),
    Do,
    Dont,
}
impl Instruction {
    fn multiply(&self, process: Instruction) -> (Instruction, u32) {
        match (self, process.clone()) {
            (Instruction::Mul((x, y)), Instruction::Do) => (Instruction::Do, x * y),
            (Instruction::Dont, _) => (Instruction::Dont, 0),
            (Instruction::Do, _) => (Instruction::Do, 0),
            (_, Instruction::Dont) => (Instruction::Dont, 0),
            _ => (process, 0),
        }
    }
}

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

fn do_or_dont(memory: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't")),
        value(Instruction::Do, tag("do")),
        parse_instruction,
    ))(memory)
}

pub fn mul_2(prog_mem: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, do_or_dont).map(|(_, ins)| ins))(prog_mem)
}
pub fn part2() -> u32 {
    let data = get_daily_input(2024, 3);
    let (_, data) = mul_2(data.as_str()).unwrap();
    let (_, total) = data
        .into_iter()
        .fold((Instruction::Do, 0), |(process, totes), instruct| {
            let (process, value) = instruct.multiply(process);
            (process, totes + value)
        });
    total
}

#[cfg(test)]
mod tests {
    // use crate::*;

    use crate::part2::{mul_2, Instruction};

    #[test]
    fn test_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (_, data) = mul_2(input).unwrap();
        let (_, total) = data
            .iter()
            .fold((Instruction::Do, 0), |(process, totes), instruct| {
                println!("{:?} and process is: {:?}", &instruct, &process);
                let (process, value) = instruct.multiply(process);
                (process, totes + value)
            });
        assert_eq!(total, 48);
    }
}
