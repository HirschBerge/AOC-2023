use {{crate_name}}::{*};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]

#[divan::bench]
fn part1() {
    println!("Hello World");
}

#[divan::bench]
fn part2() {
    println!("Hello World");
}
