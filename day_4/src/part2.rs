use glam::IVec2;

use crate::get_daily_input;
use crate::process_data;

const DIRECTIONS: [[IVec2; 2]; 4] = [
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
];

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

pub fn part2() -> usize {
    let data = get_daily_input(2024, 4);
    get_xmas(data.clone())
    // data
    // todo!("working on it");
}

#[cfg(test)]
mod tests {
    // use crate::*;
    use crate::part2::get_xmas;

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
