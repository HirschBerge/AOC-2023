use aochelpers::get_daily_input;
use std::error::Error;

const WORDVALUES: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(1, 2023)?;
    println!("Part 1 result: {}", answer(&data, false));
    println!("Part 2 result: {}", answer(&data, true));

    Ok(())
}

fn answer(data: &str, part2: bool) -> i32 {
    let mut result = 0;
    for line in data.split('\n') {
        result += first_number(line, part2) * 10 + last_number(line, part2)
    }
    result
}

fn first_number(s: &str, part2: bool) -> i32 {
    if s.is_empty() {
        0
    } else if let Some(v) = s.chars().next().unwrap_or('z').to_digit(10) {
        v as i32
    } else if let Some(v) = WORDVALUES.iter().find_map(|(w, v)| {
        if part2 && s.starts_with(w) {
            Some(v)
        } else {
            None
        }
    }) {
        *v
    } else {
        first_number(&s[1..], part2)
    }
}

fn last_number(s: &str, part2: bool) -> i32 {
    if s.is_empty() {
        0
    } else if let Some(v) = s.chars().last().unwrap_or('z').to_digit(10) {
        v as i32
    } else if let Some(v) = WORDVALUES.iter().find_map(|(w, v)| {
        if part2 && s.ends_with(w) {
            Some(v)
        } else {
            None
        }
    }) {
        *v
    } else {
        last_number(&s[..s.len() - 1], part2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_number_part1() {
        assert_eq!(first_number("1abc2", false), 1);
        assert_eq!(first_number("pqr3stu8vwx", false), 3);
        assert_eq!(first_number("a1b2c3d4e5f", false), 1);
        assert_eq!(first_number("treb7uchet", false), 7);
    }
    #[test]
    fn test_last_number_part1() {
        assert_eq!(last_number("1abc2", false), 2);
        assert_eq!(last_number("pqr3stu8vwx", false), 8);
        assert_eq!(last_number("a1b2c3d4e5f", false), 5);
        assert_eq!(last_number("treb7uchet", false), 7);
    }
    #[test]
    fn test_first_number_part2() {
        assert_eq!(first_number("two1nine", true), 2);
        assert_eq!(first_number("eightwothree", true), 8);
        assert_eq!(first_number("abcone2threexyz", true), 1);
        assert_eq!(first_number("abcone2threexyz", true), 1);
        assert_eq!(first_number("abcone2threexyz", true), 1);
        assert_eq!(first_number("xtwone3four", true), 2);
        assert_eq!(first_number("abcone2threexyz", true), 1);
        assert_eq!(first_number("4nineeightseven2", true), 4);
        assert_eq!(first_number("abcone2threexyz", true), 1);
        assert_eq!(first_number("abcone2threexyz", true), 1);
        assert_eq!(first_number("zoneight234", true), 1);
        assert_eq!(first_number("abcone2threexyz", true), 1);
        assert_eq!(first_number("7pqrstsixteen", true), 7);
    }

    #[test]
    fn test_part1() {
        let data: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let part1_res = answer(data, false);
        assert_eq!(part1_res, 142);
    }

    #[test]
    fn test_part2() {
        let data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let part1_res = answer(data, true);
        assert_eq!(part1_res, 281);
    }
}
