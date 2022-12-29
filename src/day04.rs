use aoc_runner_derive::aoc;

fn section_tuple(inp: &str) -> (i32, i32) {
    let sections: Vec<&str> = inp.split('-').collect();
    (
        sections[0].parse::<i32>().unwrap(),
        sections[1].parse::<i32>().unwrap(),
    )
}

fn contains(left: (i32, i32), right: (i32, i32)) -> bool {
    /* Does left contain right? */
    right.0 >= left.0 && right.1 <= left.1
}

#[aoc(day4, part1)]
fn part1(content: &str) -> i32 {
    content
        .lines()
        .map(|line| line.split(',').collect())
        .map(|sections: Vec<&str>| {
            let left = section_tuple(sections[0]);
            let right = section_tuple(sections[1]);
            (contains(left, right) || contains(right, left)) as i32
        })
        .sum()
    // 453
}

fn overlap(left: (i32, i32), right: (i32, i32)) -> bool {
    /* Do left and right overlap? */
    contains(left, right)
        || contains(right, left)
        || (left.0 < right.0 && (left.1 >= right.0 && left.1 <= right.1))
        || (left.1 > right.1 && (left.0 >= right.0 && left.0 <= right.1))
}

#[aoc(day4, part2)]
fn part2(content: &str) -> i32 {
    content
        .lines()
        .map(|line| line.split(',').collect())
        .map(|sections: Vec<&str>| {
            let left = section_tuple(sections[0]);
            let right = section_tuple(sections[1]);
            (overlap(left, right)) as i32
        })
        .sum()
    // 919
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 4);
    }
}
