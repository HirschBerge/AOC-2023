use day_13::parse;
use divan::Bencher;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn bench_part1(bencher: Bencher) {
    let input = include_str!("../input");
    let (_, parsed_data) = match parse(input) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("unable to parse: {}", e);
            std::process::exit(1);
        }
    };
    bencher.bench(|| {
    let total = parsed_data
        .iter()
        .fold(0, |acc, machine| acc + day_13::part1::solve_machine(machine));
    total
    });
}

#[divan::bench]
fn bench_part2(bencher: Bencher) {
    let input = include_str!("../input");
    let (_, parsed_data) = match parse(input) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("unable to parse: {}", e);
            std::process::exit(1);
        }
    };
    bencher.bench(|| {
    let total = parsed_data
        .iter()
        .fold(0, |acc, machine| acc + day_13::part2::solve_machine(machine));
    total
    });
}
#[divan::bench]
fn bench_parsing() {
    let input = include_str!("../input");
    let _ = parse(input);
}
