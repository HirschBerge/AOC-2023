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
use std::str::FromStr;
use std::{collections::HashMap, error::Error};

pub fn gather_data() -> Option<(Vec<i32>, Vec<i32>)> {
    let answer = match get_input() {
        Ok(ans) => ans,
        Err(e) => {
            eprintln!("Failed to get input for today with: {}", e);
            return None;
        }
    };
    let result = return_sorted_nums(answer.as_str());

    let (left_list, right_list) = match result {
        Ok((_, (left_ids, right_ids))) => (left_ids, right_ids),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return None;
        }
    };
    Some((left_list, right_list))
}

// INFO: Just grabs the data from AOC
pub fn get_input() -> Result<String, Box<dyn Error>> {
    let data = get_daily_input(1, 2024)?;
    Ok(data)
}

// NOTE: Handling input data

pub fn nom_into_nums(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(
        line_ending,
        separated_pair(
            map_res(take_while1(|c: char| c.is_ascii_digit()), i32::from_str),
            space1,
            map_res(take_while1(|c: char| c.is_ascii_digit()), i32::from_str),
        ),
    )(input)
}
