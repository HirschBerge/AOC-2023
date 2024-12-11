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

use std::collections::HashMap;

use nom::character;
pub fn score_stone(stone: i64, depth: u8, cachemap: &mut HashMap<(i64, u8), i64>) -> i64 {
    if depth == 0 {
        return 1;
    }
    if let Some(&cached_result) = cachemap.get(&(stone, depth)) {
        return cached_result;
    }
    let str_stone = stone.to_string();
    let result = if stone == 0 {
        score_stone(1, depth - 1, cachemap)
    } else if str_stone.len() % 2 == 0 {
        let digits = (stone as f64).log10().floor() as u32 + 1;
        let divisor = 10i64.pow(digits / 2);

        let left = stone / divisor;
        let right = stone % divisor;
        score_stone(left, depth - 1, cachemap) + score_stone(right, depth - 1, cachemap)
    } else {
        return score_stone(stone * 2024, depth - 1, cachemap);
    };

    cachemap.insert((stone, depth), result);
    result
}

// OPTIMIZE: Sort, cache answers, get count of repeated numbers, for i in repeated.len() vec.push()
pub fn score_all_stones(stones: Vec<i64>, blinks: u8) -> i64 {
    let mut lookup: HashMap<(i64, u8), i64> = HashMap::new();
    let mut answer: i64 = 0;
    for stone in stones {
        answer += score_stone(stone, blinks, &mut lookup);
    }
    answer
}

fn parsing(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, character::complete::i64)(input)
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
