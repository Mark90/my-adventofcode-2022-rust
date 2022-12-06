use aoc2022::utils;
use std::{collections::HashSet, fs::read_to_string};

const DAY: &str = "day06";

fn get_start_marker(content: &str, marker_size: usize) -> i32 {
    let chars = content
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    for position in (marker_size - 1)..chars.len() {
        let window: HashSet<&char> = chars[position - (marker_size - 1)..position + 1]
            .into_iter()
            .collect();
        if window.len() == marker_size {
            return (position as i32) + 1;
        }
    }
    unreachable!("No solution");
}
fn part1(content: &str) -> i32 {
    get_start_marker(content, 4)
}

fn part2(content: &str) -> i32 {
    get_start_marker(content, 14)
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 1538
    println!("part2 {}", part2(&content)); // 2315
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 11);
    }

    #[test]
    fn test_part_2() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part2(&content), 26);
    }
}
