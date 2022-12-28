use aoc2022::utils;
use std::{collections::HashSet, fs::read_to_string};

const DAY: &str = "day14";

fn part1(content: &str) -> i32 {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    for blockline in content.lines() {
        let coords: Vec<(i32, i32)> = blockline
            .split(" -> ")
            .map(|e| e.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .collect();
        let mut prev = coords.first().unwrap();
        for coord in coords.iter().skip(1) {
            // println!("Draw from {:?} to {:?}", prev, coord);
            let xmin = coord.0.min(prev.0);
            let xmax = coord.0.max(prev.0);
            let ymin = coord.1.min(prev.1);
            let ymax = coord.1.max(prev.1);
            for x in xmin..=xmax {
                for y in ymin..=ymax {
                    let boo = (x, y);
                    // println!("Inserting {boo:?}");
                    grid.insert(boo);
                }
            }
            prev = coord;
        }
    }

    let grid_xmin = grid.iter().map(|(x, _)| *x).min().unwrap();
    let grid_xmax = grid.iter().map(|(x, _)| *x).max().unwrap();
    let grid_ymin = 0;
    let grid_ymax = grid.iter().map(|(_, y)| *y).max().unwrap();

    let sand_origin = (500, 0);

    print_grid(
        grid_ymin,
        grid_ymax,
        grid_xmin,
        grid_xmax,
        sand_origin,
        &grid,
    );

    let transformations = [(0, 1), (-1, 1), (1, 1)]; // down, down+left, down+right

    let mut settled_sand: HashSet<(i32, i32)> = HashSet::new();
    let mut settled = true;
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
                settled_sand.insert(sand);
                break;
            }

            // Successfully moved
            if sand.1 >= grid_ymax {
                // Overlowing
                break;
            }
        }
    }

    settled_sand.iter().len() as i32
}

fn print_grid(
    grid_ymin: i32,
    grid_ymax: i32,
    grid_xmin: i32,
    grid_xmax: i32,
    sand_origin: (i32, i32),
    grid: &HashSet<(i32, i32)>,
) {
    for y in grid_ymin..=grid_ymax {
        for x in grid_xmin..=grid_xmax {
            if (x, y) == sand_origin {
                print!("+");
            } else if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn part2(content: &str) -> i32 {
    0
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 799
                                           // println!("part2 {}", part2(&content));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 24);
    }

    // #[test]
    // fn test_part_2() {
    //     let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
    //     assert_eq!(part2(&content), 140);
    // }
}
