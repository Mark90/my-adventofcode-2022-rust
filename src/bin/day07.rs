use aoc2022::utils;
use std::{collections::HashMap, fs::read_to_string};

use crate::Node::*;

const DAY: &str = "day07";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Node {
    File(String, u32),
    Directory(String, Vec<Node>),
}

impl Node {
    fn mkdir(dirname: &str) -> Node {
        Directory(dirname.to_string(), vec![])
    }

    fn add_file(&mut self, filename: &str, size: u32) {
        if let Directory(_dirname, ref mut wrapped_vector) = self {
            wrapped_vector.push(File(filename.to_string(), size));
        }
    }
    fn add_node(&mut self, node: Node) {
        if let Directory(_dirname, ref mut wrapped_vector) = self {
            wrapped_vector.push(node);
        }
    }

    fn getsize(&self, dir_sizes: &mut Vec<(String, u32)>) -> u32 {
        match self {
            Directory(_dirname, wrapped_vector) => {
                let total_size = wrapped_vector
                    .iter()
                    .map(|node| node.getsize(dir_sizes))
                    .sum();
                dir_sizes.push((_dirname.to_string(), total_size));
                total_size
            }
            File(_filename, filesize) => *filesize,
        }
    }
}

fn enter_dir(parent: &mut Node, output_iter: &mut std::iter::Skip<std::str::Lines>) {
    let mut listing = false;
    loop {
        match output_iter.next() {
            None => return, // Exhausted the terminal output
            Some(line) => {
                let lineparts: Vec<&str> = line.split_whitespace().collect();

                match lineparts[..] {
                    ["$", "cd", ".."] => return, // Back up to parent
                    ["$", "cd", directory] => {
                        listing = false;
                        let mut subdirectory = Node::mkdir(directory);
                        enter_dir(&mut subdirectory, output_iter); // Recurse into subdir to add nodes
                        parent.add_node(subdirectory); // Finally move subdir reference into parent dir
                    }
                    ["$", "ls"] => listing = true,
                    ["dir", _directory] => continue, // Don't create directory now to avoid borrowing hell
                    [filesize, filename] => {
                        if !listing {
                            // Sanity check
                            unreachable!("Parsed ls output while not listing!?")
                        }
                        parent.add_file(filename, filesize.parse::<u32>().unwrap());
                    }
                    _ => unreachable!("Unhandled line format"),
                }
            }
        }
    }
}

fn parse_terminal_output(content: &str) -> Node {
    let mut root = Node::mkdir("/");

    let mut output_iter = content.lines().skip(1);
    enter_dir(&mut root, &mut output_iter);

    root
}

fn part1(content: &str) -> u32 {
    // Parse the file structure
    let root = parse_terminal_output(content);

    // Gather directory sizes
    let mut dir_sizes: Vec<(String, u32)> = Vec::new();
    root.getsize(&mut dir_sizes);

    // Return sum of directory sizes <= 100_000
    dir_sizes
        .iter()
        .map(|(_dirname, dirsize)| *dirsize)
        .filter(|size| *size <= 100_000)
        .sum::<u32>()
}

fn part2(content: &str) -> i32 {
    //
    0
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 919137
                                           // println!("part2 {}", part2(&content));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 95437);
    }

    // #[test]
    // fn test_part_2() {
    //     let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
    //     assert_eq!(part2(&content), 140);
    // }
}
