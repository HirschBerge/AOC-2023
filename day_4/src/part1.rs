use std::collections::HashMap;

use crate::get_daily_input;
use crate::DIRECTIONS;
use glam::IVec2;

fn process_data(data: String) -> HashMap<IVec2, char> {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>()
}

fn get_xmas(input: String) -> usize {
    let positions = process_data(input);
    let mas = ['M', 'A', 'S'];
    positions
        .iter()
        .filter(|(_, value)| **value == 'X')
        .map(|(position, _)| {
            let count = DIRECTIONS
                .iter()
                .map(|mas_positions| {
                    mas_positions
                        .iter()
                        .map(|pos| positions.get(&(position + pos)))
                        .enumerate()
                        .all(|(index, value)| mas.get(index) == value)
                })
                .filter(|b| *b)
                .count();
            count
        })
        .sum()
}

pub fn part1() -> usize {
    let data = get_daily_input(2024, 4);
    get_xmas(data.clone())
    // data
    // todo!("working on it");
}

#[cfg(test)]
mod tests {
    // use crate::*;

    use part1::get_xmas;

    use crate::*;

    #[test]
    fn test_one() {
        let data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = get_xmas(data.to_owned());
        assert_eq!(result, 18);
    }
}
