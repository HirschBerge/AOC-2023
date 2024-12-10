pub fn part1() -> usize {
    let data = include_str!("../input");
    let nums = vectorize_diskmap(data);
    let layout = diskmap_to_layout(nums);
    let sorted = palindrome_sort(layout);
    generate_checksum(sorted)
}

pub fn vectorize_diskmap(input: &str) -> Vec<u32> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
}

pub fn diskmap_to_layout(diskmaps: Vec<u32>) -> Vec<String> {
    let mut final_layout = vec![];
    let mut current_id = 0;
    diskmaps.into_iter().enumerate().for_each(|(idx, file)| {
        if idx % 2 == 0 {
            for _ in 0..file {
                final_layout.push(current_id.to_string());
            }
            current_id += 1;
        } else {
            for _ in 0..file {
                final_layout.push(".".to_string())
            }
        }
    });
    final_layout
}

fn palindrome_sort(mut unsorted_layout: Vec<String>) -> Vec<String> {
    let mut i = 0;
    let mut j = unsorted_layout.len() - 1;

    while i < j {
        let a = &unsorted_layout[i];
        let b = &unsorted_layout[j];

        match (a.contains("."), b.contains('.')) {
            (true, false) => {
                unsorted_layout.swap(i, j);
                i += 1;
            }
            (true, true) => {
                j -= 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    unsorted_layout
}

pub fn generate_checksum(sorted_layout: Vec<String>) -> usize {
    sorted_layout
        .iter()
        .enumerate()
        .filter_map(|(idx, value)| match value.parse::<usize>() {
            Ok(val) => Some(idx * val),
            Err(_) => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    // use crate::*;

    use crate::part1::{diskmap_to_layout, generate_checksum, palindrome_sort, vectorize_diskmap};

    #[test]
    fn test_one() {
        let input = "2333133121414131402";
        let nums = vectorize_diskmap(input);
        let layout = diskmap_to_layout(nums);
        let sorted = palindrome_sort(layout);
        let ans = generate_checksum(sorted);
        println!("{:?}", ans);
        assert_eq!(ans, 1928);
    }
    #[test]
    fn parse_into_nums() {
        let input = "2333133121414131402";
        let nums = vectorize_diskmap(input);
        let expected: Vec<u32> = [2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2].to_vec();
        assert_eq!(expected, nums);
    }
    #[test]
    fn diskmap_to_layout_test() {
        let expected = [
            "0".to_string(),
            "0".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            "2".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            "3".to_string(),
            "3".to_string(),
            "3".to_string(),
            ".".to_string(),
            "4".to_string(),
            "4".to_string(),
            ".".to_string(),
            "5".to_string(),
            "5".to_string(),
            "5".to_string(),
            "5".to_string(),
            ".".to_string(),
            "6".to_string(),
            "6".to_string(),
            "6".to_string(),
            "6".to_string(),
            ".".to_string(),
            "7".to_string(),
            "7".to_string(),
            "7".to_string(),
            ".".to_string(),
            "8".to_string(),
            "8".to_string(),
            "8".to_string(),
            "8".to_string(),
            "9".to_string(),
            "9".to_string(),
        ]
        .to_vec();
        let input = "2333133121414131402";
        let nums = vectorize_diskmap(input);
        let layout = diskmap_to_layout(nums);
        assert_eq!(expected, layout);
    }
}
