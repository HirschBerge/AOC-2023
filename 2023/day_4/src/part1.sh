#![allow(unused_imports)]
#![allow(dead_code)]
use aochelpers::get_daily_input;
use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0, space1},
    // complete::tag,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
    Parser,
};
use std::{collections::HashSet, error::Error};

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    my_cards: HashSet<u32>,
}
impl Card {
    fn powa(&self) -> u32 {
        let power = self.winning.intersection(&self.my_cards).count() as u32;
        if power == 0 {
            power
        } else {
            2u32.pow(power - 1)
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(4, 2023)?;
    println!("{}", descriptive_name(data).unwrap()); // for line in data.lines() {
                                                     //     let tacos = parse_cards(line);
                                                     //     dbg!(tacos);
                                                     // }
    Ok(())
}
fn descriptive_name(data: String) -> Result<String, Box<dyn Error>> {
    let (_, card_data) = parse_cards(&data).expect("A valid parse");
    let result = card_data.iter().map(|card| card.powa()).sum::<u32>();
    Ok(result.to_string())
    // Ok("33".to_string())
}
fn gen_cards(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(create_set, tuple((tag("|"), space1)), create_set)
        .map(|(winning, my_cards)| Card { winning, my_cards })
        .parse(input)
}
fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, gen_cards).parse(input)
}

fn create_set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )
    .parse(input)
}

// fn return_cards(input: &str) -> IResult<(), Box<dyn Error>> {
//     let (input, _) = delimited(tag("Card :"), digit1, tag(": "))(input)?;
//     // println!("{}", input);
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> Result<(), Box<dyn Error>> {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();
        assert_eq!("13".to_string(), descriptive_name(data)?);
        Ok(())
    }
}
