use aoc2022::utils;
use std::fs::read_to_string;

const DAY: &str = "day10";

#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX(i32),
}

fn part1(content: &str) -> i32 {
    let mut instructions =
        content
            .lines()
            .map(|line| match line.split(" ").collect::<Vec<&str>>()[..] {
                ["noop"] => Instruction::NOOP,
                ["addx", something] => Instruction::ADDX(something.parse::<i32>().unwrap()),
                _ => unreachable!("imp"),
            });

    let check_cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut sums = Vec::new();

    let mut x = 1;
    let mut cycle = 0;

    let mut pending_addx = None;
    let mut addx_value = 0;

    loop {
        cycle += 1;
        if check_cycles.contains(&cycle) {
            sums.push(x * cycle);
        }

        if pending_addx == Some(cycle) {
            x += addx_value;
            pending_addx = None;
        } else if !pending_addx.is_some() {
            match instructions.next() {
                Some(Instruction::NOOP) => {}
                Some(Instruction::ADDX(value)) => {
                    pending_addx = Some(cycle + 1);
                    addx_value = value;
                }
                None => return sums.iter().sum(),
            }
        }
    }
}

fn part2(content: &str) -> i32 {
    0
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 15260
    println!("part2 {}", part2(&content));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 13140);
    }

    // #[test]
    // fn test_part_2() {
    //     let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
    //     assert_eq!(part2(&content), 45000);
    // }
}
