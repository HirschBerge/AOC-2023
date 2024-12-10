use aochelpers::get_daily_input;
use itertools::Itertools;
use std::{collections::BTreeMap, error::Error};
fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(3, 2023)?;
    let p1_answer = process(data).unwrap();
    println!("{p1_answer}");
    Ok(())
}
#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}
pub fn process(input: String) -> Result<u32, Box<dyn Error>> {
    let mut total = 0;
    // Map out the coordinates of the locations for each character according to Value above.
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| {
            line.chars().enumerate().map(move |(x, character)| {
                (
                    (idx as i32, x as i32),
                    match character {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => {
                            Value::Number(c.to_digit(10).expect("This should be a number."))
                        }
                        c => Value::Symbol(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<(i32, i32), Value>>();
    // Create a vec with just location and value of each non-empty number through various match
    // statements
    let mut coords: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match coords.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = coords.iter_mut().last().expect("should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                coords.push(vec![((*x, *y), *num)]);
                            }
                        }
                        None => todo!(),
                    }
                }
                None => coords.push(vec![((*x, *y), *num)]),
            }
            // println!("{x},{y}");
        }
    }
    for num_list in coords {
        //(x,y)
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let num_positions: Vec<(i32, i32)> = num_list.iter().map(|((y, x), _)| (*x, *y)).collect();
        let pos_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions.iter().map(|outer_pos| {
                    // outer_pos.x + pos.x .y + .yu
                    (outer_pos.0 + pos.1, outer_pos.1 + pos.0)
                })
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();
        let is_part_num = pos_to_check.iter().any(|pos| {
            let value = map.get(pos);
            // #[allow(clippy::match_like_matches_macro)]
            if let Some(Value::Symbol(_)) = value {
                true
            } else {
                false
            }
        });
        if is_part_num {
            total += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        }
    }
    // dbg!(total);
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
