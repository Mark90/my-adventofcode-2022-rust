use aoc2022::utils;
use std::fs::read_to_string;

const DAY: &str = "day04";

fn section_tuple(inp: &str) -> (i32, i32) {
    let sections: Vec<&str> = inp.split('-').collect();
    (
        sections[0].parse::<i32>().unwrap(),
        sections[1].parse::<i32>().unwrap(),
    )
}

fn contains(left: (i32, i32), right: (i32, i32)) -> bool {
    /* Does left contain right? */
    right.0 >= left.0 && right.1 <= left.1
}

fn part1(content: &str) -> i32 {
    content
        .lines()
        .map(|line| line.split(',').collect())
        .map(|sections: Vec<&str>| {
            let left = section_tuple(sections[0]);
            let right = section_tuple(sections[1]);
            (contains(left, right) || contains(right, left)) as i32
        })
        .sum()
}

fn overlap(left: (i32, i32), right: (i32, i32)) -> bool {
    /* Do left and right overlap? */
    contains(left, right)
        || contains(right, left)
        || (left.0 < right.0 && (left.1 >= right.0 && left.1 <= right.1))
        || (left.1 > right.1 && (left.0 >= right.0 && left.0 <= right.1))
}

fn part2(content: &str) -> i32 {
    content
        .lines()
        .map(|line| line.split(',').collect())
        .map(|sections: Vec<&str>| {
            let left = section_tuple(sections[0]);
            let right = section_tuple(sections[1]);
            (overlap(left, right)) as i32
        })
        .sum()
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 453
    println!("part2 {}", part2(&content)); // 919
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 2);
    }

    #[test]
    fn test_part_2() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part2(&content), 4);
    }
}
