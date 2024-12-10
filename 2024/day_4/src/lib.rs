use glam::IVec2;
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
use std::{collections::HashMap, str::FromStr};

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
];

fn process_data(data: String) -> HashMap<IVec2, char> {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>()
}

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
