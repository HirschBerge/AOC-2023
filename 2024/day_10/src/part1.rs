use std::collections::HashMap;

use glam::IVec2;
use nom::{
    character::complete::{line_ending, satisfy},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};
use pathfinding::prelude::*;

pub fn part1() -> &'static str {
    let data = include_str!("../input");
    kinda_main(data.into());
    "Temp"
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

pub type Span<'a> = LocatedSpan<&'a str>;
fn alphanum_pos(input: Span) -> IResult<Span, (IVec2, u32)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_numeric())(input)?;
    Ok((input, (IVec2::new(x, y), c.to_digit(10).unwrap())))
}

pub fn parse(input: Span) -> IResult<Span, HashMap<IVec2, u32>> {
    let (input, lines) = separated_list1(line_ending, many1(alphanum_pos))(input)?;

    let hashmap = lines
        .iter()
        .flatten()
        .copied()
        .collect::<HashMap<IVec2, u32>>();

    Ok((input, hashmap))
}
pub fn kinda_main(input: Span) {
    let trailheads: HashMap<IVec2, u32> = match parse(input) {
        Ok((_, locations)) => locations
            .into_iter()
            .filter(|(_, height)| *height == 0)
            .collect(),
        Err(e) => {
            eprintln!("Uh oh! {}", e);
            std::process::exit(1);
        }
    };

    dbg!(trailheads);
}

#[cfg(test)]
mod tests {
    // use crate::*;

    use crate::part1::{kinda_main, Span};

    #[test]
    fn test_process() {
        let input = Span::from(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        kinda_main(input);
        assert_eq!(1, 99);
    }
}
