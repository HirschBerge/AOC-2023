use core::time;
use std::collections::HashMap;

use glam::IVec2;

use crate::get_daily_input;

fn move_up(pos: IVec2) -> IVec2 {
    pos + IVec2::new(0, -1)
}

fn move_down(pos: IVec2) -> IVec2 {
    pos + IVec2::new(0, 1)
}

fn move_left(pos: IVec2) -> IVec2 {
    pos + IVec2::new(-1, 0)
}

fn move_right(pos: IVec2) -> IVec2 {
    pos + IVec2::new(1, 0)
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn process_data(data: String) -> HashMap<IVec2, char> {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>()
}

fn running(input: HashMap<IVec2, char>) -> usize {
    let starting_chars = ['^', '<', '>', 'v']; // OPTIMIZE: You could remove the chars and just make it '^' but oh well.
    let starting_char: IVec2 = input
        .iter()
        .filter_map(|(vec, chara)| {
            if starting_chars.contains(chara) {
                Some(*vec)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()[0];
    let mut moving = true;
    let mut current_item = starting_char;
    let mut locations: Vec<IVec2> = vec![current_item];
    let mut direct = Direction::North;
    while moving {
        // dbg!(input.get(&current_item));
        let mut next_item = take_step(&direct, current_item);

        match input.get(&next_item) {
            Some(next) => {
                if *next == '#' {
                    println!("Ouch! I ran into something...\nYou turn to your right, you are now facing: {:?}", direct);
                    std::thread::sleep(time::Duration::from_millis(500));
                    println!("You walk for a bit");
                    std::thread::sleep(time::Duration::from_millis(1000));

                    direct = rotate(direct);
                    next_item = take_step(&direct, current_item);
                }
                // NOTE: Next iteration
                current_item = next_item;
                if !locations.contains(&current_item) {
                    locations.push(current_item);
                }
                if locations.len() == 200000 {
                    moving = false;
                }
            }
            None => break,
        }
    }
    locations.len()
}

fn take_step(direct: &Direction, current_item: IVec2) -> IVec2 {
    match direct {
        Direction::North => move_up(current_item),
        Direction::East => move_right(current_item),
        Direction::South => move_down(current_item),
        Direction::West => move_left(current_item),
    }
}

fn rotate(direct: Direction) -> Direction {
    match direct {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

pub fn part1() -> usize {
    let data = get_daily_input(2024, 6);
    let mapped_chars = process_data(data);
    running(mapped_chars)
}

#[cfg(test)]
mod tests {
    // use crate::*;

    use crate::part1::{process_data, running};

    #[test]
    fn test_one() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();
        let processed_data = process_data(input);
        let answer = running(processed_data);
        assert_eq!(answer, 41);
    }
}
