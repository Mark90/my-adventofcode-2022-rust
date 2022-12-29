use aoc_runner_derive::aoc;

fn parse(content: &str) -> Vec<i32> {
    let mut sums: Vec<i32> = Vec::new();
    let mut current_sum: i32 = 0;
    for _line in content.lines() {
        let line = _line;
        if line.len() == 0 {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            let new_sum = line.parse::<i32>().unwrap();
            current_sum += new_sum;
        }
    }
    if current_sum > 0 {
        sums.push(current_sum);
    }
    return sums;
}

#[aoc(day1, part2)]
fn part2(content: &str) -> i32 {
    let mut sums: Vec<i32> = parse(content);
    sums.sort_by(|x, y| y.cmp(x));
    let part2_sum = sums[0] + sums[1] + sums[2];
    return part2_sum;
}

#[aoc(day1, part1)]
fn part1(content: &str) -> i32 {
    let sums: Vec<i32> = parse(content);
    let max_sum = sums.iter().max_by(|x, y| x.cmp(y)).unwrap();
    return *max_sum;
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 24000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 45000);
    }
}
