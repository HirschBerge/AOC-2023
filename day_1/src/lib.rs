use aochelpers::get_daily_input;
use nom::{
    bytes::complete::take_while1,
    character::complete::{line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
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

pub fn return_sorted_nums(input: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    let (input, lines) = nom_into_nums(input)?;
    // HACK: Create as muts so we can sort
    let (mut left_location_ids, mut right_location_ids): (Vec<i32>, Vec<i32>) =
        lines.into_iter().unzip();
    left_location_ids.sort();
    right_location_ids.sort();
    Ok((input, (left_location_ids, right_location_ids)))
}

// NOTE: Part One functions

pub fn sum_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut total_distance = 0;
    for (left_loc_id, right_loc_id) in left.iter().zip(right.iter()) {
        total_distance += (left_loc_id - right_loc_id).abs();
    }
    total_distance
}

// NOTE: Part Two functions

// INFO: We only need one sides' unique count of IDs
pub fn gen_right_hashmap(right: Vec<i32>) -> HashMap<i32, usize> {
    let mut r_loc_freq = HashMap::new();
    for r_location_id in right {
        *r_loc_freq.entry(r_location_id).or_insert(0) += 1;
    }
    r_loc_freq
}

pub fn comput_similarity(left: Vec<i32>, right: HashMap<i32, usize>) -> usize {
    let mut similarity = 0;
    left.into_iter().for_each(|location_id| {
        let freq = right.get(&location_id).copied().unwrap_or(0);
        similarity += location_id as usize * freq;
    });
    similarity
}
