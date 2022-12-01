use aoc2022::utils;

fn main() {
    let mut sums: Vec<i32> = Vec::new();
    let mut current_sum: i32 = 0;
    let lines = utils::read_lines("./inputs/day01.txt").unwrap();
    for _line in lines {
        let line = _line.unwrap();
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

    let max_sum = sums.iter().max_by(|x, y| x.cmp(y)).unwrap();
    println!("part1 {}", max_sum);

    sums.sort_by(|x, y| y.cmp(x));

    let part2_sum = sums[0] + sums[1] + sums[2];
    println!("part2 {}", part2_sum);
}
