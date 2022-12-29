use aoc_runner_derive::aoc;
use std::collections::HashSet;

fn mhdist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[aoc(day15, part1)]
fn part1(content: &str) -> u32 {
    let mut sensors: Vec<(i32, i32, i32)> = Vec::new();
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();

    let mut xmin = i32::MAX;
    let mut xmax = i32::MIN;
    for line in content.lines() {
        let hmwo = line.replace(",", "").replace(":", "").replace("=", " ");
        let s: Vec<&str> = hmwo.split_whitespace().collect();
        let sx = s[3].parse::<i32>().unwrap();
        let sy = s[5].parse::<i32>().unwrap();
        let bx = s[11].parse::<i32>().unwrap();
        let by = s[13].parse::<i32>().unwrap();
        let dist_s_to_b = mhdist((sx, sy), (bx, by));
        xmin = xmin.min(sx - dist_s_to_b);
        xmax = xmax.max(sx + dist_s_to_b);
        sensors.push((sx, sy, dist_s_to_b));
        beacons.insert((bx, by));
    }

    let y = if xmax > 100 { 2000000 } else { 10 };
    let mut positions_without_beacon = 0u32;
    for x in xmin..=xmax {
        let position = (x, y);
        if beacons.contains(&position) {
            // Already a beacon here
            continue;
        }
        // We need to know the X where a beacon cannot be present
        // So for every sensor, check if mhdist to this X,Y is equal/smaller than the sensor's dist to its closest beacon
        // If this x,y falls within that circle then there cannot be a beacon; we can stop checking other sensors
        for (sx, sy, mh_sb) in sensors.iter() {
            let dist = mhdist((*sx, *sy), position); // Distance from this X to the sensor
            if dist <= *mh_sb {
                positions_without_beacon += 1;
                break;
            }
        }
    }

    positions_without_beacon
    // 4811413
}

#[aoc(day15, part2)]
fn part2(content: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3    
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 26);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part2(&INPUT), 93);
    // }
}
