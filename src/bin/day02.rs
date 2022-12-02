use aoc2022::utils;
use std::fs::read_to_string;

const DAY: &str = "day02";

fn get_score(choice: &str) -> i32 {
    match choice {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("imp"),
    }
}

/* Possible winning combinations (tuples with our/their score) */
const WINNING: [(i32, i32); 3] = [(1, 3), (2, 1), (3, 2)];

fn game_outcome_score(their_score: i32, our_score: i32) -> i32 {
    if their_score == our_score {
        return 3;
    }
    let score = (our_score, their_score);
    if WINNING.contains(&score) {
        return 6;
    }
    return 0;
}

fn part1(content: &str) -> i32 {
    let mut total_score = 0i32;
    for line in content.lines() {
        let str_vec = line.split(' ').collect::<Vec<&str>>();
        let [their_shape, our_shape] = <[&str; 2]>::try_from(str_vec).ok().unwrap();
        let [their_score, our_score] = [get_score(their_shape), get_score(our_shape)];
        total_score += our_score + game_outcome_score(their_score, our_score);
    }
    return total_score;
}

fn part2(content: &str) -> i32 {
    let mut score = 0i32;
    for line in content.lines() {
        let str_vec = line.split(' ').collect::<Vec<&str>>();
        let [their_shape, expected_outcome] = <[&str; 2]>::try_from(str_vec).ok().unwrap();
        let their_score = get_score(their_shape);

        let opponent_win = WINNING
            .iter()
            .filter(|&&x| x.0 == their_score)
            .next()
            .unwrap();
        let opponent_loose = WINNING
            .iter()
            .filter(|&&x| x.1 == their_score)
            .next()
            .unwrap();

        let our_score = match expected_outcome {
            "X" => 0 + opponent_win.1,   // lose
            "Y" => 3 + their_score,      // draw
            "Z" => 6 + opponent_loose.0, // win
            _ => panic!("imp"),
        };
        score += our_score;
    }
    return score;
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 14297
    println!("part2 {}", part2(&content)); // 10498
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 15);
    }

    #[test]
    fn test_part_2() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part2(&content), 12);
    }
}
