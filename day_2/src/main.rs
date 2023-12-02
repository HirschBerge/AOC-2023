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

fn answer(data: String, color_totals: HashMap<&str, u32>) -> Result<u32, Box<dyn Error>> {
    let mut total_valid_sum = 0;
    for line in data.split('\n') {
        let id_re = Regex::new(r"Game (\d+)").unwrap();
        let game_id = return_game_id(line, id_re).unwrap();
        let mut invalid_game = false;
        for color in "red green blue".split_whitespace() {
            let color_re_value = format!(r"(\d+) {}", color);
            let color_re = Regex::new(&color_re_value).unwrap();
            // println!("{}", &color_re_value);
            let color_count = return_colors(line, color_re);
            // println!(
            //     "Game ID: {}, Color: {}, Count: {:?}",
            //     game_id, color, color_count
            // );

            if color_count
                .iter()
                .any(|&number| number > *color_totals.get(color).unwrap_or(&0))
            {
                println!(
                    "Invalid Game ID: {} {} was greater than {}",
                    game_id, color, color_totals[color]
                );
                invalid_game = true;
                break;
            }
        }

        if !invalid_game {
            println!("Valid Game ID: {}", game_id);
            total_valid_sum += game_id.parse::<u32>().unwrap();
        }
        // println!("{game_id}");
    }
    println!("{total_valid_sum}");
    Ok((total_valid_sum))
}
fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(2, 2023)?;
    let mut color_totals = HashMap::new();
    color_totals.insert("red", 12);
    color_totals.insert("green", 13);
    color_totals.insert("blue", 14);
    // dbg!(color_totals);
    let _ = answer(data, color_totals);
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
        assert_eq!(answer(data, color_totals).unwrap(), 8)
    }
}
