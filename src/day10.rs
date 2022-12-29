use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX(i32),
}

#[aoc(day10, part1)]
fn part1(content: &str) -> i32 {
    let mut instructions =
        content
            .lines()
            .map(|line| match line.split(" ").collect::<Vec<&str>>()[..] {
                ["noop"] => Instruction::NOOP,
                ["addx", something] => Instruction::ADDX(something.parse::<i32>().unwrap()),
                _ => unreachable!("imp"),
            });

    let check_cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut sums = Vec::new();

    let mut x = 1;
    let mut cycle = 0;

    let mut pending_addx = None;
    let mut addx_value = 0;

    loop {
        cycle += 1;
        if check_cycles.contains(&cycle) {
            sums.push(x * cycle);
        }

        if pending_addx == Some(cycle) {
            x += addx_value;
            pending_addx = None;
        } else if !pending_addx.is_some() {
            match instructions.next() {
                Some(Instruction::NOOP) => {}
                Some(Instruction::ADDX(value)) => {
                    pending_addx = Some(cycle + 1);
                    addx_value = value;
                }
                None => return sums.iter().sum(),
            }
        }
    }
    // 15260
}

#[aoc(day10, part2)]
fn part2(content: &str) -> String {
    let mut instructions =
        content
            .lines()
            .map(|line| match line.split(" ").collect::<Vec<&str>>()[..] {
                ["noop"] => Instruction::NOOP,
                ["addx", something] => Instruction::ADDX(something.parse::<i32>().unwrap()),
                _ => unreachable!("imp"),
            });
    let mut x = 1;
    let mut cycle = 0;
    let mut pixels: Vec<char> = Vec::new();

    let mut pending_addx = None;
    let mut addx_value = 0;
    while cycle < 240 {
        // draw
        if (cycle % 40) >= x - 1 && (cycle % 40) <= x + 1 {
            pixels.push('#');
        } else {
            pixels.push('.');
        }
        cycle += 1;

        if pending_addx == Some(cycle) {
            x += addx_value;
            pending_addx = None;
        } else if !pending_addx.is_some() {
            match instructions.next() {
                Some(Instruction::NOOP) => {}
                Some(Instruction::ADDX(value)) => {
                    pending_addx = Some(cycle + 1);
                    addx_value = value;
                }
                None => break,
            }
        }
    }

    let mut lines: Vec<String> = Vec::new();
    for chars in pixels.clone().chunks(40) {
        let line = chars.into_iter().collect::<String>();
        lines.push(line);
    }
    lines.join("\n")
    // PGHFGLUG
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 13140);
    }

    #[test]
    fn test_part_2() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(part2(&INPUT), expected);
    }
}
