use crate::{parsing, score_all_stones};

pub fn part2() -> i64 {
    let data = include_str!("../input");
    let (_, parsed_stones) = parsing(data).unwrap();
    score_all_stones(parsed_stones, 75)
}

// TODO: Return the score for a given stone in a manner that is easy to cache.

#[cfg(test)]
mod tests {
    // use crate::*;

    use crate::{parsing, score_all_stones};

    #[test]
    fn test_two() {
        let input = "125 17";
        let (_, parsed_stones) = parsing(input).unwrap();
        let ans = score_all_stones(parsed_stones, 75);
        assert_eq!(55312, ans);
    }
}
