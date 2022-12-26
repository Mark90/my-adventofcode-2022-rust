use aoc2022::utils;
use std::fs::read_to_string;

use crate::Item::*;

const DAY: &str = "day13";

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Number(u32),
    List(Vec<Item>),
}

impl Item {
    fn add_number(&mut self, val: u32) {
        if let List(ref mut wrapped_list) = self {
            wrapped_list.push(Number(val));
        }
    }
    fn add_list(&mut self, list: Item) {
        if let List(ref mut wrapped_list) = self {
            wrapped_list.push(list);
        }
    }
}

fn parse_pair(string_pair: &str) -> (Vec<Item>, Vec<Item>) {
    let mut packets = string_pair.lines();
    let packet1 = packets.next().unwrap();
    let packet2 = packets.next().unwrap();

    (parse_packet(packet1), parse_packet(packet2))
}

fn parse_packet(raw_packet_line: &str) -> Vec<Item> {
    let mut line_chars = raw_packet_line.chars();
    line_chars.next(); // Strip first [
    line_chars.next_back(); // Strip last ]

    // Normalize line so that we can split by whitespace
    let normalized = line_chars
        .as_str()
        .replace(",", " ")
        .replace("[", " [ ")
        .replace("]", " ] ");
    let items: Vec<&str> = normalized.split_whitespace().collect();

    // Keep track of nested Item lists
    let mut nest_level: usize = 0;
    let mut nested_lists: Vec<Item> = Vec::new();

    // Main vector of Items
    let mut result: Vec<Item> = vec![];
    for item in items {
        match item.parse::<u32>() {
            Ok(value) => {
                if nest_level == 0 {
                    // Add number to main vector
                    result.push(Number(value))
                } else {
                    // Add number to nested List
                    nested_lists[nest_level - 1].add_number(value);
                }
            }
            Err(_err) => match item {
                "[" => {
                    // Increase nesting level: initialize List
                    nest_level += 1;
                    nested_lists.push(List(vec![]));
                }
                "]" => {
                    // Decrease nesting level: add last List to...
                    nest_level -= 1;
                    if nest_level == 0 {
                        // ... the main vector
                        result.push(nested_lists.pop().unwrap());
                    } else {
                        // ... or the List 1 level above
                        let popped_list = nested_lists.pop().unwrap();
                        nested_lists[nest_level - 1].add_list(popped_list);
                    }
                }
                _ => unreachable!("imp"),
            },
        }
    }

    result
}

fn part1(content: &str) -> i32 {
    // parse the input into pairs
    let mut pairs: Vec<(Vec<Item>, Vec<Item>)> = Vec::new();
    let string_pairs = content.split("\n\n");
    for string_pair in string_pairs {
        let parsed: (Vec<Item>, Vec<Item>) = parse_pair(string_pair);
        pairs.push(parsed);
    }

    // Find pairs in the right order
    let mut sum_of_correct_pair_indices = 0;

    for (idx, pair) in pairs.iter().enumerate() {
        match in_right_order(pair.0.clone(), pair.1.clone()) {
            Some(true) => sum_of_correct_pair_indices += (idx as i32) + 1,
            Some(false) => continue,
            None => unreachable!("imp"),
        }
    }

    sum_of_correct_pair_indices
}

fn in_right_order(left: Vec<Item>, right: Vec<Item>) -> Option<bool> {
    let mut items_left = left.iter();
    let mut items_right = right.iter();

    loop {
        let l = items_left.next();
        let r = items_right.next();
        match (l, r) {
            (Some(Item::Number(_value)), Some(Item::List(items))) => {
                // Mixed integer / list: convert left to a list and recurse
                let converted_left = vec![(*l.unwrap()).clone()];
                match in_right_order(converted_left, (*items).clone()) {
                    None => continue,
                    Some(result) => return Some(result),
                }
            }
            (Some(Item::List(items)), Some(Item::Number(_value))) => {
                // Mixed integer / list: convert right to a list and recurse
                let converted_right = vec![(*r.unwrap()).clone()];
                match in_right_order((*items).clone(), converted_right) {
                    None => continue,
                    Some(result) => return Some(result),
                }
            }
            (Some(Item::List(items_left)), Some(Item::List(items_right))) => {
                // 2 lists: recurse
                match in_right_order((*items_left).clone(), (*items_right).clone()) {
                    None => continue,
                    Some(result) => return Some(result),
                }
            }
            (None, Some(_item)) => {
                // Left list runs out of items -> in the right order
                return Some(true);
            }
            (Some(_item), None) => {
                // Right list runs out of items -> NOT in the right order
                return Some(false);
            }
            (Some(Item::Number(val1)), Some(Item::Number(val2))) => {
                if val1 == val2 {
                    continue; // Identical number in both lists, no decision
                }
                return Some(val1 < val2); // Different number, lower should come first
            }
            (None, None) => {
                // Both lists same length and equal values, no decision
                return None;
            }
        }
    }
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 6076
                                           // println!("part2\n{}\n", part2(&content));
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 13);
    }

    // #[test]
    // fn test_part_2() {
    //     let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
    //     assert_eq!(part2(&content), 13);
    // }

    #[test]
    fn test_1_correct() {
        /*
        [1,1,3,1,1]
        [1,1,5,1,1]
        */
        let packet1 = vec![List([1, 1, 3, 1, 1].map(|n| Number(n)).to_vec())];
        let packet2 = vec![List([1, 1, 5, 1, 1].map(|n| Number(n)).to_vec())];

        assert_eq!(in_right_order(packet1, packet2), Some(true));
    }

    #[test]
    fn test_2_correct() {
        /*
        [[1],[2,3,4]]
        [[1],4]
        */
        let packet1 = vec![
            List(vec![Number(1)]),
            List([2, 3, 4].map(|n| Number(n)).to_vec()),
        ];
        let packet2 = vec![List(vec![Number(1)]), Number(4)];

        assert_eq!(in_right_order(packet1, packet2), Some(true));
    }

    #[test]
    fn test_3_incorrect() {
        /*
        [9]
        [[8,7,6]]
        */
        let packet1 = vec![
            List(vec![Number(9)]),
            List([8, 7, 6].map(|n| Number(n)).to_vec()),
        ];
        let packet2 = vec![List(vec![Number(1)]), Number(4)];

        assert_eq!(in_right_order(packet1, packet2), Some(false));
    }

    #[test]
    fn test_4_correct() {
        /*
        [[4,4],4,4]
        [[4,4],4,4,4]
        */
        let packet1 = vec![List(vec![Number(4), Number(4)]), Number(4), Number(4)];
        let packet2 = vec![
            List(vec![Number(4), Number(4)]),
            Number(4),
            Number(4),
            Number(4),
        ];

        assert_eq!(in_right_order(packet1, packet2), Some(true));
    }

    #[test]
    fn test_5_incorrect() {
        /*
        [7,7,7,7]
        [7,7,7]
        */
        let packet1 = vec![List([7, 7, 7, 7].map(|n| Number(n)).to_vec())];
        let packet2 = vec![List([7, 7, 7].map(|n| Number(n)).to_vec())];

        assert_eq!(in_right_order(packet1, packet2), Some(false));
    }

    #[test]
    fn test_6_correct() {
        /*
        []
        [3]
        */
        let packet1 = vec![];
        let packet2 = vec![Number(3)];

        assert_eq!(in_right_order(packet1, packet2), Some(true));
    }

    #[test]
    fn test_7_incorrect() {
        /*
        [[[]]]
        [[]]
        */
        let packet1 = vec![List(vec![List(vec![])])];
        let packet2 = vec![List(vec![])];

        assert_eq!(in_right_order(packet1, packet2), Some(false));
    }

    #[test]
    fn test_8_incorrect() {
        /*
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
        */

        let packet1 = vec![
            Number(1),
            List(vec![
                Number(2),
                List(vec![
                    Number(3),
                    List(vec![Number(4), List(vec![Number(5), Number(6), Number(7)])]),
                ]),
            ]),
            Number(8),
            Number(9),
        ];

        let packet2 = vec![
            Number(1),
            List(vec![
                Number(2),
                List(vec![
                    Number(3),
                    List(vec![Number(4), List(vec![Number(5), Number(6), Number(0)])]),
                ]),
            ]),
            Number(8),
            Number(9),
        ];

        assert_eq!(in_right_order(packet1, packet2), Some(false));
    }

    #[test]
    fn test_parse_line_nested() {
        let input = "[10,[2,[3,[4,[5,6,7]]]],83,9]";
        let expected_result = vec![
            Number(10),
            List(vec![
                Number(2),
                List(vec![
                    Number(3),
                    List(vec![Number(4), List(vec![Number(5), Number(6), Number(7)])]),
                ]),
            ]),
            Number(83),
            Number(9),
        ];
        let actual_result = parse_packet(input);
        assert_eq!(actual_result, expected_result);
    }
}
