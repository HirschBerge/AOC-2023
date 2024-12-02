use day_2::generate_reports;
use day_2::part1::check_all_safe_p1;
use day_2::part2::process;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let reports = generate_reports();
    let _ = check_all_safe_p1(reports);
}

#[divan::bench]
fn part2() {
    let reports = generate_reports();
    let _ = process(reports.as_str());
}
