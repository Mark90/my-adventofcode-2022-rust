use aoc2022::utils;
use std::{collections::HashSet, fs::read_to_string};

const DAY: &str = "day09";

#[derive(Debug, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn update_tail(head: &Coord, tail: &mut Coord) {
    let diff_x = head.x - tail.x;
    let diff_y = head.y - tail.y;

    if diff_x.abs() == 2 || diff_y.abs() == 2 {
        if diff_x != 0 {
            tail.x += 1.min(diff_x.abs()) * (diff_x.abs() / diff_x);
        }
        if diff_y != 0 {
            tail.y += 1.min(diff_y.abs()) * (diff_y.abs() / diff_y);
        }
    }
}

fn up(head: &mut Coord) {
    head.y += -1;
}
fn down(head: &mut Coord) {
    head.y += 1;
}
fn left(head: &mut Coord) {
    head.x += -1;
}
fn right(head: &mut Coord) {
    head.x += 1;
}

fn part1(content: &str) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };
    visited.insert(tail.as_tuple());

    for line in content.lines() {
        let mut l = line.split(" ");
        let dir = l.next().unwrap();
        let count = l.next().unwrap().parse::<i32>().unwrap();
        let f = match dir {
            "R" => right,
            "L" => left,
            "U" => up,
            "D" => down,
            _ => unreachable!("imp"),
        };
        for _ in 0..count {
            f(&mut head);
            update_tail(&head, &mut tail);
            visited.insert(tail.as_tuple());
        }
    }

    visited.len() as i32
}

fn part2(content: &str) -> i32 {
    0
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 6494
    println!("part2 {}", part2(&content));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 13);
    }

    #[test]
    fn test_update_tail_diag1() {
        let head = Coord { x: 2, y: 1 };
        let mut tail = Coord { x: 1, y: 3 };
        update_tail(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 2 });
    }
    #[test]
    fn test_update_tail_diag2() {
        let head = Coord { x: 2, y: 0 };
        let mut tail = Coord { x: 4, y: 1 };
        update_tail(&head, &mut tail);
        assert_eq!(tail, Coord { x: 3, y: 0 });
    }
    #[test]
    fn test_update_tail_right() {
        let head = Coord { x: 3, y: 1 };
        let mut tail = Coord { x: 1, y: 1 };
        update_tail(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 1 });
    }
    #[test]
    fn test_update_tail_noop1() {
        let head = Coord { x: 3, y: 1 };
        let mut tail = Coord { x: 2, y: 1 };
        update_tail(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 1 });
    }
    #[test]
    fn test_update_tail_noop2() {
        let head = Coord { x: 2, y: 1 };
        let mut tail = Coord { x: 2, y: 1 };
        update_tail(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 1 });
    }
    #[test]
    fn test_update_tail_diag1_neg() {
        let head = Coord { x: 2, y: -1 };
        let mut tail = Coord { x: 1, y: 1 };
        update_tail(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 0 });
    }

    #[test]
    fn test_update_tail_down_negative() {
        let head = Coord { x: 2, y: 0 };
        let mut tail = Coord { x: 2, y: -2 };
        update_tail(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: -1 });
    }

    #[test]
    fn test_part_2() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part2(&content), 36);
    }
}
