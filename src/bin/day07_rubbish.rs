use aoc2022::utils;
use std::rc::{Rc, Weak};
use std::{cell::RefCell, fs::read_to_string};

const DAY: &str = "day07";

struct File {
    name: String,
    size: i32,
    // parent: Directory,
}
struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
    calculated_size: i32,
    // parent: RefCell<Weak<Directory>>,
}

// impl<'a> Node<'a> {
//     fn list(&self, level: Option<i32>) {
//         let leve = level.unwrap_or(0);
//         let pref = " ".repeat((4 * leve) as usize);
//         println!("{}Children for {}:", pref, self.name);
//         for i in &self.children {
//             println!("{}- child {}", pref, i.name);
//             i.list(Some(leve + 1));
//         }
//     }
// }

// fn mkdir(name: String, parent: Directory) -> Directory {
fn mkdir(name: &str) -> Directory {
    let dir = Directory {
        name: String::from(name),
        calculated_size: 0,
        directories: Vec::new(),
        files: Vec::new(),
        // parent: RefCell::new(Weak::new()),
    };
    dir
}

fn mkfile(name: &str, size: i32) -> File {
    File {
        name: String::from(name),
        size,
    }
}

fn part1(content: &str) -> i32 {
    // let mut leafs: Vec<&File> = Vec::new();
    // let mut root = Directory {
    //     name: String::from("root"),
    //     calculated_size: 0,
    //     directories: Vec::new(),
    //     files: Vec::new(),
    //     // parent: RefCell::new(Default::default()),
    // };
    let mut root = mkdir("root");

    // let pwd = &root;
    // let mut pwd = stack.pop().unwrap();
    let mut pwd = root;

    let mut stack = Vec::from([]);

    for line in content.split("$ ").skip(1) {
        let mut mdlines = line.lines();
        let cmd: &str = mdlines.next().unwrap();
        match &cmd[2..4] {
            "cd" => {
                let dir = &cmd[5..];
                match dir {
                    ".." => {
                        pwd = stack.pop().unwrap();
                    }
                    _ => {
                        stack.push(pwd);
                        for d in pwd.directories {
                            if d.name == dir {
                                pwd = d;
                                break;
                            }
                        }
                    }
                }
            }
            "ls" => {
                for node in mdlines {
                    let a = node.split(" ").collect::<Vec<&str>>();
                    match a[0] {
                        "dir" => {
                            let newdir = mkdir(a[1]);
                            // todo
                        }
                        _ => {
                            let size = a[0].parse::<i32>().unwrap();
                            let newfile = mkfile(a[1], size);
                            // let mut boo = *stack.last().unwrap();
                            pwd.files.push(newfile); // TODO fix borrow
                                                     // stack.last().unwrap()files.push(newfile);
                        }
                    }
                }
            }
            _ => unreachable!("imp"),
        }
    }
    0
}

fn part2(content: &str) -> i32 {
    0
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); //
    println!("part2 {}", part2(&content)); //
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
    //     assert_eq!(part2(&content), 26);
    // }
}
