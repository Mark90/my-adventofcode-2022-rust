use aoc2022::utils;
use std::fs::read_to_string;

use std::collections::{HashMap, HashSet};

const DAY: &str = "day08";

fn check_row(
    y: usize,
    gridsize: usize,
    all_trees: &HashMap<(i32, i32), u32>,
    visible_trees: &mut HashSet<(i32, i32)>,
) {
    let mut max_height = all_trees.get(&(0, y as i32)).unwrap();

    // left to right
    for x in 1..(gridsize - 1) {
        let next_tree = (x as i32, y as i32);
        let height = all_trees.get(&next_tree).unwrap();
        if height > max_height {
            max_height = height;
            visible_trees.insert(next_tree);
        }
    }

    // right to left
    max_height = all_trees.get(&(gridsize as i32 - 1, y as i32)).unwrap();
    for x in (1..(gridsize - 1)).rev() {
        let next_tree = (x as i32, y as i32);
        let height = all_trees.get(&next_tree).unwrap();
        if height > max_height {
            max_height = height;
            visible_trees.insert(next_tree);
        }
    }
}

fn check_column(
    x: usize,
    gridsize: usize,
    all_trees: &HashMap<(i32, i32), u32>,
    visible_trees: &mut HashSet<(i32, i32)>,
) {
    // top down
    let mut max_height = all_trees.get(&(x as i32, 0)).unwrap();
    for y in 1..(gridsize - 1) {
        let next_tree = (x as i32, y as i32);
        let height = all_trees.get(&next_tree).unwrap();
        if height > max_height {
            max_height = height;
            visible_trees.insert(next_tree);
        }
    }

    // bottom up
    max_height = all_trees.get(&(x as i32, gridsize as i32 - 1)).unwrap();
    for y in (1..(gridsize - 1)).rev() {
        let next_tree = (x as i32, y as i32);
        let height = all_trees.get(&next_tree).unwrap();
        if height > max_height {
            max_height = height;
            visible_trees.insert(next_tree);
        }
    }
}

fn part1(content: &str) -> i32 {
    let mut all_trees: HashMap<(i32, i32), u32> = HashMap::new();
    let mut visible_trees: HashSet<(i32, i32)> = HashSet::new();
    let gridsize = content.lines().next().unwrap().len();

    for (y, line) in content.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if x % (gridsize - 1) == 0 || y % (gridsize - 1) == 0 {
                visible_trees.insert((x as i32, y as i32));
            }
            all_trees.insert((x as i32, y as i32), char.to_digit(10).unwrap());
        }
    }

    // horizontal
    for y in 1..(gridsize - 1) {
        check_row(y, gridsize, &all_trees, &mut visible_trees);
    }

    // vertical
    for x in 1..(gridsize - 1) {
        check_column(x, gridsize, &all_trees, &mut visible_trees);
    }

    visible_trees.len() as i32
}

fn up(x: i32, y: i32, _n: i32) -> Option<(i32, i32)> {
    if y == 0 {
        return None;
    }
    return Some((x, y - 1));
}
fn down(x: i32, y: i32, n: i32) -> Option<(i32, i32)> {
    if y == (n - 1) {
        return None;
    }
    return Some((x, y + 1));
}
fn right(x: i32, y: i32, n: i32) -> Option<(i32, i32)> {
    if x == (n - 1) {
        return None;
    }
    return Some((x + 1, y));
}
fn left(x: i32, y: i32, _n: i32) -> Option<(i32, i32)> {
    if x == 0 {
        return None;
    }
    return Some((x - 1, y));
}

fn get_viewing_distance(
    trees: &HashMap<(i32, i32), u32>,
    from_x: i32,
    from_y: i32,
    gridsize: i32,
    direction: &dyn Fn(i32, i32, i32) -> Option<(i32, i32)>,
) -> i32 {
    let mut result = 0;
    let mut curr_x = from_x;
    let mut curr_y = from_y;
    let from_tree = trees.get(&(from_x, from_y)).unwrap();
    loop {
        match direction(curr_x, curr_y, gridsize) {
            Some(another_tree) => {
                result += 1;
                if trees.get(&another_tree).unwrap() >= from_tree {
                    break;
                }
                curr_x = another_tree.0;
                curr_y = another_tree.1;
            }
            None => break,
        }
    }
    result
}

fn scenic_score(all_trees: &HashMap<(i32, i32), u32>, gridsize: i32, x: i32, y: i32) -> i32 {
    [
        get_viewing_distance(&all_trees, x, y, gridsize, &up),
        get_viewing_distance(&all_trees, x, y, gridsize, &left),
        get_viewing_distance(&all_trees, x, y, gridsize, &right),
        get_viewing_distance(&all_trees, x, y, gridsize, &down),
    ]
    .iter()
    .copied()
    .reduce(|x, y| x * y)
    .unwrap()
}

fn part2(content: &str) -> i32 {
    let mut trees: HashMap<(i32, i32), u32> = HashMap::new();
    let gridsize = content.lines().next().unwrap().len() as i32;

    for (y, line) in content.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            trees.insert((x as i32, y as i32), char.to_digit(10).unwrap());
        }
    }

    let mut max_score = 0;
    for y in 1..(gridsize - 1) {
        for x in 1..(gridsize - 1) {
            max_score = max_score.max(scenic_score(&trees, gridsize, x, y));
        }
    }
    max_score
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 1789
    println!("part2 {}", part2(&content)); // 314820
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 21);
    }

    #[test]
    fn test_part_2() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part2(&content), 8);
    }
}
