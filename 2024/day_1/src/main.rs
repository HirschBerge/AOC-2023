use day_1::*;

fn main() {
    let (left_list, right_list) = match gather_data() {
        Some((left, right)) => (left, right),
        None => {
            eprintln!("Failed to gather data...");
            std::process::exit(1); // Exiting here will prevent the assignment to (left_list, right_list)
        }
    };
    let part = 1;
    if part == 1 {
        let total_diff = sum_distance(left_list, right_list);
        println!("Part one answer: {}", total_diff);
    } else {
        let right_hash = gen_right_hashmap(right_list);
        let similarity = comput_similarity(left_list, right_hash);
        println!("Part two answer: {}", similarity);
    }
}

#[cfg(test)]
mod tests {
    use crate::{comput_similarity, gen_right_hashmap, return_sorted_nums, sum_distance};

    #[test]
    fn test_parse_input() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let expected_output = (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]);
        let result = return_sorted_nums(input);
        assert_eq!(result, Ok(("", expected_output)));
    }
    #[test]
    fn test_sum_total() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let (_, (left, right)) = return_sorted_nums(input).unwrap();
        let total = sum_distance(left, right);
        assert_eq!(total, 11);
    }
    #[test]
    fn test_similarity() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let (_, (left, right)) = return_sorted_nums(input).unwrap();
        let right_hash = gen_right_hashmap(right);
        let similarity = comput_similarity(left, right_hash);
        assert_eq!(similarity, 31);
    }
}
