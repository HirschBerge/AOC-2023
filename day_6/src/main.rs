use nom::{
    bytes::complete::is_not,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = aochelpers::get_daily_input(6, 2023)?;
    println!("{}", get_race_details(&data));
    Ok(())
}

fn get_race_details(input: &str) -> usize {
    let (_, (time, dist)) = parse_race(input).expect("A Valid parse");
    let mut total = 0;
    // Loop over each possible second
    for held in 0..time {
        // Get the distance for each
        let potential_dist = held * (time - held);
        // If it wins
        if potential_dist > dist {
            // Add one for each that we haven't done yet. This allows us to skip to the end
            total += time - held;
            // total += 1;
            // Skip to the end.
            for rev in (held..time).rev() {
                let p_dist = rev * (time - rev);
                if p_dist <= dist {
                    // If it's smaller at the end, make up for assuming.
                    total -= 1;
                // Once we get back to the range where it's winning again, break from both loops.
                // We're done. This saved us about 20ms. I will show benchmarks in a ReadMe.md
                } else {
                    break;
                }
            }
            break;
        }
    }
    total
}
fn get_times(input: &str) -> IResult<&str, usize> {
    let temp = is_not("0123456789")
        .precedes(
            separated_list1(space1, digit1)
                .map(|list| list.join("").parse::<usize>().expect("A valid parse")),
        )
        .parse(input);
    // dbg!(&temp);
    temp
}

fn parse_race(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(get_times, line_ending, get_times).parse(input)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() -> Result<(), Box<dyn std::error::Error>> {
        let data = "Time:      7  15   30
Distance:  9  40  200"
            .to_string();
        assert_eq!(71503, get_race_details(&data));
        Ok(())
    }
    #[test]
    fn part_2() -> Result<(), Box<dyn std::error::Error>> {
        let data = aochelpers::get_daily_input(6, 2023)?;
        assert_eq!(40651271, get_race_details(&data));
        Ok(())
    }
}
