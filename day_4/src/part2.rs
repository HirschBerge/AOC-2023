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
    let remaining_chars = ['M', 'S'];
    positions
        .iter()
        // Checks the letter you're looking for
        .filter(|(_, value)| **value == 'A')
        .filter(|(position, _)| {
            DIRECTIONS
                .iter()
                // Gets the possible locations
                .map(|mas_positions| {
                    mas_positions
                        .iter()
                        .map(|offset| positions.get(&(*position + offset)))
                        .enumerate()
                        .all(|(index, value)| remaining_chars.get(index) == value)
                })
                .filter(|b| *b)
                .count()
                // NOTE:
                // There are only two possible methods of maxing an 'X' with three letters
                == 2
        })
        // Now count up the total times that it returns 2
        .count()
}

pub fn part2() -> usize {
    let data = get_daily_input(2024, 4);
    get_xmas(data)
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
        assert_eq!(result, 9);
    }
}
