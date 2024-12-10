use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1, u32},
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

fn get_race_details(input: &str) -> u32 {
    let (_, (time, dist)) = parse_race(input).expect("A Valid parse");
    let mut total = 1;
    dbg!(&time, &dist);
    // Loop through both, using their length
    for i in 0..time.len() {
        let this_time = time[i];
        let this_dist = dist[i];
        let mut times_beaten = 0;
        // Loop through each second possible and multiply the time held by the dist
        for held in 0..this_time {
            let potential_dist = held * (this_time - held);
            if potential_dist > this_dist {
                println!(
                    "Winner! {} beat the record of {}",
                    potential_dist, this_dist
                );
                times_beaten += 1;
            }
        }
        total *= times_beaten
    }
    total
}
fn get_times(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn parse_race(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
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
        assert_eq!(288, get_race_details(&data));
        Ok(())
    }
}
