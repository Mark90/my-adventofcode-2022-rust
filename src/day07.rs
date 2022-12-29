use aoc_runner_derive::aoc;

use Node::*;

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

    fn get_directory_sizes(&self, dir_sizes: &mut Vec<(String, u32)>) -> u32 {
        // Recurses through nodes, storing every directory's calculated size in `dir_sizes`
        match self {
            Directory(_dirname, wrapped_vector) => {
                let total_size = wrapped_vector
                    .iter()
                    .map(|node| node.get_directory_sizes(dir_sizes))
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

#[aoc(day7, part1)]
fn part1(content: &str) -> u32 {
    // Parse the file structure
    let root = parse_terminal_output(content);

    // Gather directory sizes
    let mut dir_sizes: Vec<(String, u32)> = Vec::new();
    root.get_directory_sizes(&mut dir_sizes);

    // Return sum of directory sizes <= 100_000
    dir_sizes
        .iter()
        .map(|(_dirname, dirsize)| *dirsize)
        .filter(|size| *size <= 100_000)
        .sum::<u32>()
    // 919137
}

#[aoc(day7, part2)]
fn part2(content: &str) -> u32 {
    let total_diskspace: u32 = 70000000;
    let required_free_diskspace: u32 = 30000000;

    // Parse the file structure
    let root = parse_terminal_output(content);

    // Gather directory sizes
    let mut dir_sizes: Vec<(String, u32)> = Vec::new();
    root.get_directory_sizes(&mut dir_sizes);

    // Determine used diskspace
    let root_size = dir_sizes
        .iter()
        .filter(|(dirname, _dirsize)| dirname == "/")
        .next()
        .unwrap();

    let current_free_diskspace = total_diskspace - root_size.1;

    // Determine the diskspace to be freed, and the smallest directory to fulfil this
    let extra_space_needed = required_free_diskspace - current_free_diskspace;
    dir_sizes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let directory_to_remove = dir_sizes
        .iter()
        .filter(|(_dirname, dirsize)| dirsize >= &extra_space_needed)
        .next_back()
        .unwrap();

    directory_to_remove.1
    // 2877389
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 95437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 24933642);
    }
}
