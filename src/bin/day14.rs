use aoc2022::utils;
use std::{collections::HashSet, fs::read_to_string};

const DAY: &str = "day14";

fn parse_grid(content: &str) -> HashSet<(i32, i32)> {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    for blockline in content.lines() {
        let coords: Vec<(i32, i32)> = blockline
            .split(" -> ")
            .map(|e| e.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .collect();
        let mut prev = coords.first().unwrap();
        for coord in coords.iter().skip(1) {
            let xmin = coord.0.min(prev.0);
            let xmax = coord.0.max(prev.0);
            let ymin = coord.1.min(prev.1);
            let ymax = coord.1.max(prev.1);
            for x in xmin..=xmax {
                for y in ymin..=ymax {
                    grid.insert((x, y));
                }
            }
            prev = coord;
        }
    }
    grid
}

fn part1(content: &str) -> i32 {
    let mut grid = parse_grid(content);

    let grid_ymax = grid.iter().map(|(_, y)| *y).max().unwrap();

    let sand_origin = (500, 0);

    let transformations = [(0, 1), (-1, 1), (1, 1)]; // down, down+left, down+right

    let mut settled = true;
    let mut blocks_settled = 0;
    // Spawn sand blocks until they're not settling anymore (i.e. overflowing)
    while settled {
        let mut sand = sand_origin.clone();
        if grid.contains(&(sand.0, sand.1 + 1)) {
            // sanity check
            panic!("Block right below sand origin is blocked!?");
        }
        settled = false;

        // Move block until it settles or overflows
        loop {
            let mut moved = false;
            for i in transformations.iter() {
                let newpos = (sand.0 + i.0, sand.1 + i.1);
                if !grid.contains(&newpos) {
                    // Can move here, do it
                    sand = newpos;
                    moved = true;
                    break;
                }
            }

            if !moved {
                // Didn't move -> settled
                settled = true;
                grid.insert(sand);
                blocks_settled += 1;
                break;
            }

            // Successfully moved
            if sand.1 >= grid_ymax {
                // Overflowing
                break;
            }
        }
    }

    blocks_settled
}

fn part2(content: &str) -> i32 {
    let mut grid = parse_grid(content);

    let grid_ymax = grid.iter().map(|(_, y)| *y).max().unwrap();

    let sand_origin = (500, 0);

    let transformations = [(0, 1), (-1, 1), (1, 1)]; // down, down+left, down+right

    // Spawn sand blocks until the spawn contains settled sand
    let mut blocks_settled = 0;
    while !grid.contains(&sand_origin) {
        let mut sand = (500, 0);

        // Move block until it settles; either on a block, sand, or the floor
        loop {
            let mut moved = false;

            // let mut new_position = sand;
            for transformation in transformations.iter() {
                let newposition = (sand.0 + transformation.0, sand.1 + transformation.1);
                if !grid.contains(&newposition) && newposition.1 < (grid_ymax + 2) {
                    // There is no block on this new position, and we are not at the "floor" yet
                    sand = newposition;
                    moved = true;
                    break;
                }
            }

            if !moved {
                // Didn't move -> settled
                grid.insert(sand);
                blocks_settled += 1;
                break;
            }
        }
    }

    blocks_settled
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 799
    println!("part2 {}", part2(&content)); // 29076
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 24);
    }

    #[test]
    fn test_part_2() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part2(&content), 93);
    }
}
