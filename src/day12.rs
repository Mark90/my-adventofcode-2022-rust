use aoc_runner_derive::aoc;

use std::collections::{BinaryHeap, HashMap, HashSet};

fn get_neighbor(position: &(i32, i32), direction: i32) -> (i32, i32) {
    match direction {
        0 => (position.0, position.1 - 1), // up
        1 => (position.0 + 1, position.1), // right
        2 => (position.0, position.1 + 1), // down
        3 => (position.0 - 1, position.1), // left
        _ => unreachable!("imp"),
    }
}

fn reconstruct_path(
    came_from: &HashMap<(i32, i32), (i32, i32)>,
    current: &(i32, i32),
) -> Vec<(i32, i32)> {
    let mut total_path = Vec::new();
    let mut node = current;
    total_path.push(*node);
    loop {
        match came_from.get(&node) {
            Some(parent) => {
                total_path.push(*parent);
                node = parent;
            }
            None => break,
        }
    }
    return total_path;
}

// A* finds a path from start to goal.
// h is the heuristic function. h(n) estimates the cost to reach goal from node n.
// Rustified pseudocode from Wikipedia :-)
fn a_star<H: Fn((i32, i32)) -> i32>(
    start: (i32, i32),
    goal: (i32, i32),
    h: H,
    width: i32,
    height: i32,
    square_heights: &HashMap<(i32, i32), i32>,
) -> Option<Vec<(i32, i32)>> {
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    let mut open_set = BinaryHeap::new();
    open_set.push(start);
    // Also store discovered nodes in a set because binaryheap doesn't have a containment check
    let mut open_set2 = HashSet::new();
    open_set2.insert(start);

    // For node n, came_from[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    // For node n, g_score[n] is the cost of the cheapest path from start to n currently known.
    // g_score := map with default value of Infinity
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();
    g_score.insert(start, 0);

    // For node n, f_score[n] := g_score[n] + h(n). f_score[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    // f_score := map with default value of Infinity
    let mut f_score: HashMap<(i32, i32), i32> = HashMap::new();
    f_score.insert(start, h(start));

    while !open_set.is_empty() {
        // This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
        // current := the node in openSet having the lowest f_score[] value
        let current = open_set.pop().unwrap();
        open_set2.remove(&current);

        if current == goal {
            return Some(reconstruct_path(&came_from, &current));
        }

        // for each neighbor of current
        let mut direction = 0;
        while direction < 4 {
            let neighbor = get_neighbor(&current, direction);
            direction += 1;
            if neighbor.0 < 0 || neighbor.0 == width || neighbor.1 < 0 || neighbor.1 == height {
                // Out of bounds
                continue;
            }
            if square_heights.get(&current).unwrap()
                < &((square_heights.get(&neighbor).unwrap()) - 1)
            {
                // Neighbor is too high to reach
                continue;
            }

            // d(current,neighbor) is the weight of the edge from current to neighbor -> hardcoded to 1 in this puzzle
            // tentative_g_score is the distance from start to the neighbor through current
            // tentative_g_score := g_score[current] + d(current, neighbor)
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            if &tentative_g_score < g_score.get(&neighbor).unwrap_or(&(width * height)) {
                // This path to neighbor is better than any previous one. Record it!
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(neighbor));

                if !open_set2.contains(&neighbor) {
                    open_set.push(neighbor);
                    open_set2.insert(neighbor);
                }
            }
        }
    }

    None
}

#[aoc(day12, part1)]
fn part1(content: &str) -> i32 {
    // Parse the input
    let mut square_heights: HashMap<(i32, i32), i32> = HashMap::new();
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let height = content.lines().count() as i32;
    let width = content.lines().next().unwrap().len() as i32;
    for (y, line) in content.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let position = (x as i32, y as i32);
            match char {
                'S' => {
                    start = position;
                    square_heights.insert(position, 'a' as i32);
                }
                'E' => {
                    goal = position;
                    square_heights.insert(position, 'z' as i32);
                }
                _ => {
                    square_heights.insert(position, char as i32);
                }
            };
        }
    }

    // Heuristic function using Manhattan Distance to the goal square
    let heuristic = |node: (i32, i32)| ((node.0 - goal.0).abs() + (node.1 - goal.1).abs());

    // Use A* to find shortest path from S to E
    let shortest_path = a_star(start, goal, heuristic, width, height, &square_heights).unwrap();
    shortest_path.len() as i32 - 1
    // 425
}

#[aoc(day12, part2)]
fn part2(content: &str) -> i32 {
    // Parse the input, this time keeping an array of all possible start squares
    let mut square_heights: HashMap<(i32, i32), i32> = HashMap::new();
    let mut start_squares = Vec::new();
    let mut goal = (0, 0);
    let height = content.lines().count() as i32;
    let width = content.lines().next().unwrap().len() as i32;
    for (y, line) in content.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let position = (x as i32, y as i32);
            match char {
                'a' => {
                    start_squares.push(position);
                    square_heights.insert(position, char as i32);
                }
                'E' => {
                    goal = position;
                    square_heights.insert(position, 'z' as i32);
                }
                _ => {
                    square_heights.insert(position, char as i32);
                }
            };
        }
    }

    // Heuristic function using Manhattan Distance to the goal square
    let heuristic = |node: (i32, i32)| ((node.0 - goal.0).abs() + (node.1 - goal.1).abs());

    // Use A* to find shortest path from each start square to E, return the lowest
    // Not very efficient but still completes in a few seconds on a 5 year old laptop with an i5
    let mut overall_shortest_path_length = width * height;
    for start in start_squares {
        match a_star(start, goal, heuristic, width, height, &square_heights) {
            Some(shortest_path) => {
                overall_shortest_path_length =
                    overall_shortest_path_length.min(shortest_path.len() as i32 - 1);
            }
            None => {}
        }
    }

    overall_shortest_path_length
    // 418
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 29);
    }
}
