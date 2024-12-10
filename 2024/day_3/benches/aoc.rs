use day_3::{part1::part1, part2::part2};

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

#[divan::bench]
fn bench_lolz() {
    let file = include_str!("../../.inputs/2024/3");
    let mut answer = 0;

    for i in 1..1000 {
        for j in 1..1000 {
            let pattern = format!("mul({},{})", i, j);
            let times_seen: Vec<&str> = file.matches(pattern.as_str()).collect();
            let times = times_seen.len();

            if times != 0 {
                answer += times * i * j
            }
        }
    }
    println!("Your answer is: {}", answer);
}
