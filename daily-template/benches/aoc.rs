use {{crate_name}}::{comput_similarity, gather_data, gen_right_hashmap, sum_distance};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let (left_list, right_list) = match gather_data() {
        Some((left, right)) => (left, right),
        None => {
            eprintln!("Failed to gather data...");
            std::process::exit(1); // Exiting here will prevent the assignment to (left_list, right_list)
        }
    };
    let _ = sum_distance(left_list, right_list);
}

#[divan::bench]
fn part2() {
    let (left_list, right_list) = match gather_data() {
        Some((left, right)) => (left, right),
        None => {
            eprintln!("Failed to gather data...");
            std::process::exit(1); // Exiting here will prevent the assignment to (left_list, right_list)
        }
    };
    let right_hash = gen_right_hashmap(right_list);
    let _ = comput_similarity(left_list, right_hash);
}
