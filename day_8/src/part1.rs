use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::take_till, character::complete::satisfy, multi::many1, sequence::preceded,
    AsChar, IResult,
};
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

pub fn part1() -> usize {
    let data = include_str!("../input");
    process(Span::new(data))
}

fn process(input: Span) -> usize {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let y_bound = 0i32..height as i32;
    let x_bound = 0i32..width as i32;
    let (_input, mut processed_data) = process_data(input).unwrap();
    processed_data.sort_by(|a, b| a.1.cmp(&b.1));
    let results = processed_data
        .chunk_by(|a, b| a.1 == b.1)
        .flat_map(|chunk| {
            chunk
                .iter()
                .combinations(2)
                .flat_map(|pairs| {
                    let diff = pairs[0].0 - pairs[1].0;
                    [pairs[0].0 + diff, pairs[1].0 - diff]
                })
                .filter(|x| x_bound.contains(&x.x) && y_bound.contains(&x.y))
        })
        .unique()
        .count();
    results
}

fn process_data(input: Span) -> IResult<Span, Vec<(IVec2, char)>> {
    many1(preceded(take_till(|c: char| c.is_alphanum()), locate_pos))(input)
}

fn locate_pos(input: Span) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_alphanum())(input)?;
    Ok((input, (IVec2::new(x, y), c)))
}

#[cfg(test)]
mod tests {
    // use crate::*;

    use crate::part1::{process, Span};

    #[test]
    fn test_one() {
        let input = Span::new(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        let ans = process(input);
        assert_eq!(ans, 14);
    }
}
