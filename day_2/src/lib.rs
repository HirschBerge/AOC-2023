use aochelpers::get_daily_input;
use nom::{
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::process::exit;
pub mod part1;
pub mod part2;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum SafetyLevel {
    Safe,
    Unsafe,
}

impl SafetyLevel {
    #[allow(dead_code)]
    pub fn set_safe(&mut self) {
        *self = SafetyLevel::Safe;
    }

    pub fn set_unsafe(&mut self) {
        *self = SafetyLevel::Unsafe;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UpOrDown {
    Basis,
    Raising,
    Lowering,
}

// INFO: Just grabs the data from AOC
pub fn get_input() -> Result<String, Box<dyn Error>> {
    let data = get_daily_input(2, 2024)?;
    Ok(data)
}

// NOTE: Handling input data

pub fn nom_into_nums(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(
        line_ending,
        separated_list1(space1, map_res(digit1, i32::from_str)),
    )(input)
}

pub fn generate_reports() -> String {
    match get_input() {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Could not get data successfully with error: {}", e);
            exit(1);
        }
    }
}
