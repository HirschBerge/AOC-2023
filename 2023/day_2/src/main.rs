use aochelpers::get_daily_input;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

fn return_game_id(text: &str, re: Regex) -> Option<&str> {
    if let Some(filt) = re.captures(text) {
        filt.get(1).map(|m| m.as_str())
    } else {
        None
    }
}

fn return_colors(txt: &str, re: Regex) -> Vec<u32> {
    re.captures_iter(txt)
        .filter_map(|captures| captures.get(1).map(|m| m.as_str().parse::<u32>().ok()))
        .flatten()
        .collect()
}

fn answer(data: String, color_totals: HashMap<&str, u32>, p2: bool) -> Result<u32, Box<dyn Error>> {
    let mut total_valid_sum = 0;
    let mut p2_sum: u32 = 0;
    for line in data.split('\n') {
        let id_re = Regex::new(r"Game (\d+)").unwrap();
        let game_id = return_game_id(line, id_re).unwrap();
        let mut invalid_game = false;
        let mut power: u32 = 1;
        for color in "red green blue".split_whitespace() {
            let color_re_value = format!(r"(\d+) {}", color);
            let color_re = Regex::new(&color_re_value).unwrap();
            let color_count = return_colors(line, color_re);
            if p2 {
                let max = color_count.iter().max().unwrap();
                power *= max;
            }
            if color_count
                .iter()
                .any(|&number| number > *color_totals.get(color).unwrap_or(&0))
            {
                invalid_game = true;
            }
        }
        p2_sum += power;
        if !invalid_game {
            total_valid_sum += game_id.parse::<u32>().unwrap();
        }
    }
    if p2 {
        Ok(p2_sum)
    } else {
        Ok(total_valid_sum)
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(2, 2023)?;
    let mut color_totals = HashMap::new();
    color_totals.insert("red", 12);
    color_totals.insert("green", 13);
    color_totals.insert("blue", 14);
    let part = 2;
    // let part = 2;
    if 1 == part {
        println!(
            "Part 1: {:?}",
            answer(data.clone(), color_totals.clone(), false).unwrap()
        );
    } else if 2 == part {
        println!("Part 2: {:?}", answer(data, color_totals, true).unwrap());
    } else {
        panic!("Expected part 1 or 2, instead found {part}")
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_test() {
        let mut color_totals = HashMap::new();
        color_totals.insert("red", 12);
        color_totals.insert("green", 13);
        color_totals.insert("blue", 14);

        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();
        assert_eq!(answer(data, color_totals, false).unwrap(), 8)
    }
    #[test]
    fn part_2_test() {
        let mut color_totals = HashMap::new();
        color_totals.insert("red", 12);
        color_totals.insert("green", 13);
        color_totals.insert("blue", 14);

        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();
        assert_eq!(answer(data, color_totals, true).unwrap(), 2286)
    }
}
