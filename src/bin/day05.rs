use aoc2022::utils;
use std::{collections::HashMap, fs::read_to_string};

const DAY: &str = "day05";

fn parse_stacks(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut stacks: HashMap<&str, Vec<&str>> = HashMap::new();
    let drawing = input.lines();
    let stack_numbers = input.lines().last().unwrap();
    let mut column = 1;
    while column < stack_numbers.len() {
        let mut stack: Vec<&str> = Vec::new();
        // There must be a better way than this, but e
        for line in drawing.clone().rev().skip(1) {
            if column >= line.len() {
                break;
            }
            let crate_ = line.get(column..column + 1).unwrap();
            if crate_ == " " {
                break;
            }
            stack.push(crate_);
        }
        let stack_number = stack_numbers.get(column..column + 1).unwrap();
        stacks.insert(stack_number, stack);
        column += 4;
    }
    stacks
}

fn solve_it(content: &str, part1: bool) -> String {
    let input_sections = content.split("\n\n").collect::<Vec<_>>();
    assert_eq!(input_sections.len(), 2);

    let mut stacks = parse_stacks(input_sections[0]);

    let instructions = input_sections[1].lines();

    for instruction in instructions {
        let s = instruction.split(" ").collect::<Vec<&str>>();
        assert_eq!(s.len(), 6);

        let num_crates = s[1].to_string().parse::<usize>().unwrap();
        let from_stack = s[3];
        let to_stack = s[5];

        // Get the stack to move from
        let stack: &mut Vec<&str> = stacks.get_mut(from_stack).unwrap();
        // Remove crates from the stack
        let crates: Vec<&str> = stack.splice(stack.len() - num_crates.., []).collect();
        // Add crates to the target stack...
        stacks.entry(to_stack).and_modify(|entry| {
            if part1 {
                // .. in reverse order
                entry.extend(crates.iter().rev())
            } else {
                // .. in normal order
                entry.extend(crates.iter())
            }
        });
    }

    // TODO figure out the funcional approach, should be something like this
    // let res = map.values().map(|v| v.iter().take(1)).collect().join("");

    // Now for the poor man's approach
    let mut result = String::new();
    for num in 1..(stacks.len() + 1) {
        let x = num.to_string();
        result.push_str(stacks.get(&x[..]).unwrap().last().unwrap())
    }
    result
}

fn part1(content: &str) -> String {
    solve_it(content, true)
}

fn part2(content: &str) -> String {
    solve_it(content, false)
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // SHMSDGZVC
    println!("part2 {}", part2(&content)); // VRZGHDFBQ
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), "CMZ");
    }

    #[test]
    fn test_part_2() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part2(&content), "MCD");
    }
}
