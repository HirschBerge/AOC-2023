use nom::{
    bytes::complete::take_while1,
    character::complete::{line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
pub mod part1;
pub mod part2;
use std::str::FromStr;

pub fn get_daily_input(year: i32, day: i32) -> String {
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

pub fn _nom_into_nums(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(
        line_ending,
        separated_pair(
            map_res(take_while1(|c: char| c.is_ascii_digit()), i32::from_str),
            space1,
            map_res(take_while1(|c: char| c.is_ascii_digit()), i32::from_str),
        ),
    )(input)
}
