use day_2::{generate_reports, part1::check_all_safe_p1, part2::process};

fn main() {
    let reports = generate_reports();
    let p1_safeness = check_all_safe_p1(reports.clone());
    let p2_safeness = process(reports.as_str());
    println!("Part 1 answer: {}", p1_safeness);
    println!("Part 2 answer: {:?}", p2_safeness.unwrap());
}
