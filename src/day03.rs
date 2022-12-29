use aoc_runner_derive::aoc;

use std::collections::HashSet;

fn to_priority(c: char) -> i32 {
    if 'a' <= c && c <= 'z' {
        return (c as i32) - 'a' as i32 + 1;
    } else {
        return (c as i32) - 'A' as i32 + 27;
    }
}

#[aoc(day3, part1)]
fn part1(content: &str) -> i32 {
    content
        .lines()
        .map(|line| {
            let l = line.to_string();
            let (sack1, sack2) = l.split_at(l.len() / 2);
            let s1: HashSet<char> = sack1.chars().collect::<HashSet<char>>();
            let s2: HashSet<char> = sack2.chars().collect::<HashSet<char>>();
            to_priority(*s1.intersection(&s2).next().unwrap())
        })
        .sum()
    // 8139
}

#[aoc(day3, part2)]
fn part2(content: &str) -> i32 {
    let mut sacks = content
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>());

    let mut result = 0i32;
    let mut group = sacks.next().unwrap();
    let mut groupsize = 1u8;
    for sack in sacks {
        group = if groupsize == 0 {
            sack
        } else {
            group.intersection(&sack).copied().collect()
        };
        groupsize += 1;

        if groupsize == 3 {
            result += to_priority(*group.iter().next().unwrap());
            groupsize = 0;
        }
    }

    result
    // 2668
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 70);
    }
}
