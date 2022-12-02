use aoc2022::utils;
use std::fs::read_to_string;

const DAY: &str = "day01";

fn parse(content: &str) -> Vec<i32> {
    let mut sums: Vec<i32> = Vec::new();
    let mut current_sum: i32 = 0;
    for _line in content.lines() {
        let line = _line;
        if line.len() == 0 {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            let new_sum = line.parse::<i32>().unwrap();
            current_sum += new_sum;
        }
    }
    if current_sum > 0 {
        sums.push(current_sum);
    }
    return sums;
}

fn part2(content: &str) -> i32 {
    let mut sums: Vec<i32> = parse(content);
    sums.sort_by(|x, y| y.cmp(x));
    let part2_sum = sums[0] + sums[1] + sums[2];
    return part2_sum;
}

fn part1(content: &str) -> i32 {
    let sums: Vec<i32> = parse(content);
    let max_sum = sums.iter().max_by(|x, y| x.cmp(y)).unwrap();
    return *max_sum;
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content));
    println!("part2 {}", part2(&content));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 24000);
    }

    #[test]
    fn test_part_2() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part2(&content), 45000);
    }
}
