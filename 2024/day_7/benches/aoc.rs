use day_7::{part1::part1, part2::part2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn bench_part1() {

    part1();
}

#[divan::bench]
fn bench_part2() {
    part2();
}
