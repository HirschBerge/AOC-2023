use crate::{parse, Machine};

pub fn part1() -> isize {
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
    let prize = (machine.prize.x, machine.prize.y);
    let determinanterminent = machine.a.x * machine.b.y - machine.a.y * machine.b.x;
    let a = (prize.0 * machine.b.y - prize.1 * machine.b.x) / determinanterminent;
    let b = (machine.a.x * prize.1 - machine.a.y * prize.0) / determinanterminent;
    if (
        machine.a.x * a + machine.b.x * b,
        machine.a.y * a + machine.b.y * b,
    ) == (prize.0, prize.1)
    {
        (a * 3 + b) as isize
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    // use crate::*;

    use crate::part1::linear_alg;

    #[test]
    fn test_one() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let ans = linear_alg(input);
        assert_eq!(ans, 480);
    }
}
