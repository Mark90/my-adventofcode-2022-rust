use aoc2022::utils;
use std::{collections::HashSet, fs::read_to_string};

const DAY: &str = "day09";
const PART2_TAILSIZE: usize = 9;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn move_tail_to_head(head: &Coord, tail: &mut Coord) {
    /* Move tail into direction of the head */
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

fn move_up(head: &mut Coord) {
    head.y += -1;
}
fn move_down(head: &mut Coord) {
    head.y += 1;
}
fn move_left(head: &mut Coord) {
    head.x += -1;
}
fn move_right(head: &mut Coord) {
    head.x += 1;
}

fn part1(content: &str) -> i32 {
    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail.as_tuple());
    for line in content.lines() {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let steps = split.next().unwrap().parse::<i32>().unwrap();
        let move_head = match direction {
            "R" => move_right,
            "L" => move_left,
            "U" => move_up,
            "D" => move_down,
            _ => unreachable!("imp"),
        };
        for _ in 0..steps {
            move_head(&mut head);
            move_tail_to_head(&head, &mut tail);
            visited.insert(tail.as_tuple());
        }
    }

    visited.len() as i32
}

fn move_tail_to_tail(tails: &mut [Coord; PART2_TAILSIZE], tail_index: usize) {
    let head = tails[tail_index - 1];
    let mut tail = &mut tails[tail_index];
    move_tail_to_head(&head, &mut tail);
}

fn part2(content: &str) -> i32 {
    let mut head = Coord { x: 0, y: 0 };
    let mut tails = [Coord { x: 0, y: 0 }; PART2_TAILSIZE];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tails.last().unwrap().as_tuple());
    for line in content.lines() {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let steps = split.next().unwrap().parse::<i32>().unwrap();
        let move_head = match direction {
            "R" => move_right,
            "L" => move_left,
            "U" => move_up,
            "D" => move_down,
            _ => unreachable!("invalid direction"),
        };
        for _ in 0..steps {
            move_head(&mut head);
            move_tail_to_head(&head, &mut tails[0]);
            for i in 1..PART2_TAILSIZE {
                move_tail_to_tail(&mut tails, i); // Wrapper to prevent borrowing tails twice
            }
            visited.insert(tails[PART2_TAILSIZE - 1].as_tuple());
        }
    }

    visited.len() as i32
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 6494
    println!("part2 {}", part2(&content)); // 2691
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
        move_tail_to_head(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 2 });
    }
    #[test]
    fn test_update_tail_diag2() {
        let head = Coord { x: 2, y: 0 };
        let mut tail = Coord { x: 4, y: 1 };
        move_tail_to_head(&head, &mut tail);
        assert_eq!(tail, Coord { x: 3, y: 0 });
    }
    #[test]
    fn test_update_tail_right() {
        let head = Coord { x: 3, y: 1 };
        let mut tail = Coord { x: 1, y: 1 };
        move_tail_to_head(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 1 });
    }
    #[test]
    fn test_update_tail_noop1() {
        let head = Coord { x: 3, y: 1 };
        let mut tail = Coord { x: 2, y: 1 };
        move_tail_to_head(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 1 });
    }
    #[test]
    fn test_update_tail_noop2() {
        let head = Coord { x: 2, y: 1 };
        let mut tail = Coord { x: 2, y: 1 };
        move_tail_to_head(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 1 });
    }
    #[test]
    fn test_update_tail_diag1_neg() {
        let head = Coord { x: 2, y: -1 };
        let mut tail = Coord { x: 1, y: 1 };
        move_tail_to_head(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: 0 });
    }

    #[test]
    fn test_update_tail_down_negative() {
        let head = Coord { x: 2, y: 0 };
        let mut tail = Coord { x: 2, y: -2 };
        move_tail_to_head(&head, &mut tail);
        assert_eq!(tail, Coord { x: 2, y: -1 });
    }

    #[test]
    fn test_part_2() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part2(&content), 1);
        let larger_sample = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n";
        assert_eq!(part2(&larger_sample), 36);
    }
}
