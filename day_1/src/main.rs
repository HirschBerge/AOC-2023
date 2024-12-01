use aochelpers::get_daily_input;
use nom::{
    bytes::complete::take_while1,
    character::complete::{line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::error::Error;
use std::str::FromStr;

fn parse_line(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(
        line_ending,
        separated_pair(
            map_res(take_while1(|c: char| c.is_ascii_digit()), i32::from_str),
            space1,
            map_res(take_while1(|c: char| c.is_ascii_digit()), i32::from_str),
        ),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    let (input, lines) = parse_line(input)?;
    // HACK: Create as muts so we can sort
    let (mut first_numbers, mut second_numbers): (Vec<i32>, Vec<i32>) = lines.into_iter().unzip();
    first_numbers.sort();
    second_numbers.sort();
    Ok((input, (first_numbers, second_numbers)))
}

// INFO: Just grabs the data from AOC
fn get_input() -> Result<String, Box<dyn Error>> {
    let data = get_daily_input(1, 2024)?;
    Ok(data)
}

fn main() {
    let answer = match get_input() {
        Ok(ans) => ans,
        Err(e) => {
            eprintln!("Failed to get input for today with: {}", e);
            return;
        }
    };
    let result = parse_input(answer.as_str());

    let (first, second) = match result {
        Ok((_, (first_numbers, second_numbers))) => (first_numbers, second_numbers),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        }
    };
    let total_diff = sum_distance(first, second);
    println!("{}", total_diff);
}
fn sum_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut total = 0;
    for (first, second) in left.iter().zip(right.iter()) {
        total += (first - second).abs();
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, sum_distance};

    #[test]
    fn test_parse_input() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let expected_output = (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]);
        let result = parse_input(input);
        assert_eq!(result, Ok(("", expected_output)));
    }
    #[test]
    fn test_sum_total() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let (_, (left, right)) = parse_input(input).unwrap();
        let total = sum_distance(left, right);
        assert_eq!(total, 11);
    }
}
