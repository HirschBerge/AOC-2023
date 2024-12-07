use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::get_daily_input;

const OPERATORS: [char; 2] = ['*', '+'];

pub fn part1() -> u64 {
    let data = get_daily_input(2024, 7);
    let (_, set) = parse_totals(data.as_str()).unwrap();
    summing(set)
}

fn parse_totals(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse::<u64>()),
        tag(": "),
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u64>())),
    )(input)
}

fn summing(numbers: Vec<(u64, Vec<u64>)>) -> u64 {
    numbers
        .iter()
        .filter_map(|(target, numbers)| {
            let num_of_nums = numbers.len() - 1;
            (0..num_of_nums)
                .map(|_| OPERATORS)
                .multi_cartesian_product()
                .any(|sequence| {
                    let mut s = sequence.iter();
                    let answer = numbers
                        .iter()
                        .copied()
                        .reduce(|accumulator, next| match s.next().unwrap() {
                            '*' => accumulator * next,
                            '+' => accumulator + next,
                            _ => panic!("This will never happen"),
                        })
                        .unwrap();
                    *target == answer
                })
                .then_some(target)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    // use crate::*;

    use crate::part1::{parse_totals, summing};

    #[test]
    fn test_one() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let (_, set) = parse_totals(input).unwrap();
        let answer = summing(set);
        assert_eq!(answer, 3749);
    }
}
