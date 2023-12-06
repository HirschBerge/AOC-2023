use indicatif::ProgressIterator;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use std::ops::Range;
use tracing::info;

// struct SeedId(u64);

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, src: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(src_range, _)| src_range.contains(&src));

        let Some((src_range, dest_range)) =
            valid_mapping
        else {
            return src;
        };

        let offset = src - src_range.start;

        dest_range.start + offset
    }
}

#[tracing::instrument(skip(input))]
fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (dest, src, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    // dbg!(dest, num);
    Ok((input, (src..(src + num), dest..(dest + num))))
}
fn seed_map(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)).map(|mappings| SeedMap { mappings }))
        .parse(input)
}
// #[tracing::instrument(skip(input), fields(input_first_line = input.split("\n").next().unwrap()))]
fn parse_seedmaps(input: &str) -> IResult<&str, (Vec<Range<u64>>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, tag(" "), complete::u64)
                .map(|(start, offset)| (start..(start + offset))),
        ))
        .parse(input)?;
    info!(?seeds);
    let (input, maps) = many1(seed_map)(input)?;

    Ok((input, (seeds, maps)))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_, (seeds, maps)) = parse_seedmaps(input).expect("a valid parse");

    let locations = seeds
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .collect::<Vec<u64>>();
    let locations = locations
        .into_iter()
        .progress()
        .map(|seed| maps.iter().fold(seed, |seed, map| map.translate(seed)))
        .collect::<Vec<u64>>();

    Ok(locations
        .iter()
        .min()
        .expect("should have a minimum location value")
        .to_string())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aochelpers::get_daily_input(5, 2023)?;
    let input = input.as_str();
    println!("{}", process(&input).unwrap());
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        info!("here");
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
