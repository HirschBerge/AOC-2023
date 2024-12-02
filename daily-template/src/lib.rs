use aochelpers::get_daily_input;
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
use std::error::Error;
use std::{process::exit, str::FromStr};

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
    let data = get_daily_input(1, 2024)?;
    Ok(data)
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
