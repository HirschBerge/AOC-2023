use crate::{parse, Machine};

pub fn part2() -> isize {
    let data = include_str!("../input");
    linear_alg(data)
}
pub fn linear_alg(input: &str) -> isize {
    let (_, parsed_data) = match parse(input) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("unable to parse: {}", e);
            std::process::exit(1);
        }
    };
    let total = parsed_data
        .iter()
        .fold(0, |acc, machine| acc + solve_machine(machine));
    total
}
fn solve_machine(machine: &Machine) -> isize {
    let offset: isize = 10_000_000_000_000;
    let prize = (
        machine.prize.x as isize + offset,
        machine.prize.y as isize + offset,
    );
    let determinant =
        machine.a.x as isize * machine.b.y as isize - machine.a.y as isize * machine.b.x as isize;
    let a = (prize.0 * machine.b.y as isize - prize.1 * machine.b.x as isize) / determinant;
    let b = (machine.a.x as isize * prize.1 - machine.a.y as isize * prize.0) / determinant;
    if (
        machine.a.x as isize * a + machine.b.x as isize * b,
        machine.a.y as isize * a + machine.b.y as isize * b,
    ) == (prize.0, prize.1)
    {
        a * 3 + b
    } else {
        0
    }
}
