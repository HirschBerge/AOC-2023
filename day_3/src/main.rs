use day_3::{part1::part1, part2::part2};

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    // use crate::*;

    #[test]
    fn test_one() {
        assert_eq!(1, 1);
    }
}
