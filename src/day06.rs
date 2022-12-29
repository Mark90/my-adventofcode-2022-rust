use aoc_runner_derive::aoc;

use std::collections::HashSet;

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

#[aoc(day6, part1)]
fn part1(content: &str) -> i32 {
    get_start_marker(content, 4)
    // 1538
}

#[aoc(day6, part2)]
fn part2(content: &str) -> i32 {
    get_start_marker(content, 14)
    // 2315
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 26);
    }
}
