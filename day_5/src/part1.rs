use std::process::exit;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

use crate::get_daily_input;

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, nums) = separated_list1(
        line_ending,
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<u32>()),
            tag("|"),
            map_res(digit1, |s: &str| s.parse::<u32>()),
        ),
    )(input)?;
    Ok((input, nums))
}

fn parse_pages(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, _) = many1(line_ending)(input)?;
    let (input, pages) = separated_list1(
        line_ending,
        separated_list1(tag(","), map_res(digit1, |s: &str| s.parse::<u32>())),
    )(input)?;
    Ok((input, pages))
}

fn return_ordered(rules: Vec<(u32, u32)>, pages: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // -> u32 {
    let ok_pages: Vec<_> = pages
        .iter()
        .filter_map(|pages| {
            let mut current_item = pages[0];
            let mut update = &pages[1..];
            let mut before_pages: Vec<u32> = (&pages[0..0]).to_vec();
            while before_pages.len() != pages.len() {
                let must_come_after: Vec<u32> = rules
                    .iter()
                    .filter_map(|(before, after)| {
                        if *before == current_item {
                            Some(*after)
                        } else {
                            None
                        }
                    })
                    .collect();
                if !must_come_after
                    .iter()
                    .all(|page_num| !before_pages.contains(page_num))
                {
                    return None;
                }
                before_pages = (&pages[0..(before_pages.len() + 1)]).to_vec();
                // dbg!(current_item, &before_pages);

                if let Some(page) = update.first() {
                    current_item = *page;
                    update = &update[1..];
                }
            }
            Some(pages.clone())
        })
        .collect();
    ok_pages
}

fn sum_middle_values(returned_pages: Vec<Vec<u32>>) -> i32 {
    let answer = returned_pages.iter().fold(0, |ans, vec| {
        if vec.is_empty() {
            ans
        } else {
            let mid = vec.len() / 2;
            if vec.len() % 2 == 0 {
                // Even length, return the average of the middle two
                ans + (vec[mid - 1] + vec[mid]) as i32 / 2
            } else {
                // Odd length, return the middle element
                ans + vec[mid] as i32
            }
        }
    });

    if answer == 0 {
        eprintln!("Could not get sum of correct pages");
        exit(1);
    }

    answer
}

pub fn part1() -> i32 {
    let data = get_daily_input(2024, 5);
    let (unparsed_pages, rules) = match parse_rules(data.as_str()) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Uh oh... {}", e);
            exit(1);
        }
    };
    let (_, parsed_pages) = match parse_pages(unparsed_pages) {
        Ok(pages) => pages,
        Err(e) => {
            eprintln!("Dagnabbit! {}", e);
            exit(1);
        }
    };
    let ordered = return_ordered(rules, parsed_pages);
    sum_middle_values(ordered)
}

#[cfg(test)]
mod tests {
    use crate::part1::{parse_rules, sum_middle_values};

    use super::{parse_pages, return_ordered};

    #[test]
    fn test_parse_rules() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";
        let pairs = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let (_, rules) = parse_rules(input).unwrap();
        assert_eq!(pairs, rules);
    }
    #[test]
    fn test_parse_pages() {
        let input = "\n\n75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let pages: Vec<Vec<u32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let (_, parsed_pages) = parse_pages(input).unwrap();
        assert_eq!(pages, parsed_pages);
    }
    #[test]
    fn test_finding_correct_pages() {
        let rules = "29|13
47|13
47|29
47|53
47|61
53|13
53|29
61|13
61|29
61|53
75|13
75|29
75|47
75|53
75|61
97|13
97|29
97|47
97|53
97|61
97|75";
        let pages = "\n\n75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let correct_pages: Vec<Vec<u32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ];
        let (_, parsed_rules) = parse_rules(rules).unwrap();
        let (_, parsed_pages) = parse_pages(pages).unwrap();
        let returned_pages = return_ordered(parsed_rules, parsed_pages);
        assert_eq!(returned_pages, correct_pages);
    }
    #[test]
    fn test_part1() {
        let rules = "29|13
47|13
47|29
47|53
47|61
53|13
53|29
61|13
61|29
61|53
75|13
75|29
75|47
75|53
75|61
97|13
97|29
97|47
97|53
97|61
97|75";
        let pages = "\n\n75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let (_, parsed_rules) = parse_rules(rules).unwrap();
        let (_, parsed_pages) = parse_pages(pages).unwrap();
        let returned_pages = return_ordered(parsed_rules, parsed_pages);
        let answer: i32 = sum_middle_values(returned_pages);
        assert_eq!(answer, 143);
    }
}
