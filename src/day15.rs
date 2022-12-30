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
        let normalized = line.replace(",", "").replace(":", "").replace("=", " ");
        let lineparts: Vec<&str> = normalized.split_whitespace().collect();
        let sx = lineparts[3].parse::<i32>().unwrap();
        let sy = lineparts[5].parse::<i32>().unwrap();
        let bx = lineparts[11].parse::<i32>().unwrap();
        let by = lineparts[13].parse::<i32>().unwrap();
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
fn part2(content: &str) -> i64 {
    // Build list of Sensors x,y,r where r is their 'radius' (MH distance to nearest beacon)
    let mut sensors: Vec<(i32, i32, i32)> = Vec::new();

    let mut xmax = i32::MIN;
    for line in content.lines() {
        let normalized = line.replace(",", "").replace(":", "").replace("=", " ");
        let lineparts: Vec<&str> = normalized.split_whitespace().collect();
        let sx = lineparts[3].parse::<i32>().unwrap();
        let sy = lineparts[5].parse::<i32>().unwrap();
        let bx = lineparts[11].parse::<i32>().unwrap();
        let by = lineparts[13].parse::<i32>().unwrap();
        let dist_s_to_b = mhdist((sx, sy), (bx, by));
        xmax = xmax.max(sx + dist_s_to_b);
        sensors.push((sx, sy, dist_s_to_b));
    }
    sensors.sort_by(|&a, &b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    // Determine gridsize from input
    let gridsize = if xmax > 100 { 4000000 } else { 20 };

    let mut possible_beacons: HashSet<(i32, i32)> = HashSet::new();

    // Traverse grid diagonally
    for i in 0..=gridsize {
        // Check the Y-axis; for each sensor find out the range it covers on this axis (if any)
        let mut sensor_yranges: Vec<(i32, i32)> = sensors
            .iter()
            .filter_map(|sensor| get_yrange(sensor, i))
            .collect();
        // Sort and filter contained ranges
        sensor_yranges.sort_by(|&a, &b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        remove_contained_ranges(&mut sensor_yranges);

        for j in 1..sensor_yranges.len() {
            if (sensor_yranges[j].0 - sensor_yranges[j - 1].1) > 1 {
                // Found a gap to previous range
                let possible_beacon = (i as i32, sensor_yranges[j - 1].1 + 1);
                if possible_beacons.contains(&possible_beacon) {
                    return (possible_beacon.0 as i64) * 4000000_i64 + (possible_beacon.1 as i64);
                }
                possible_beacons.insert(possible_beacon);
            }
        }

        // Check the X-axis; for each sensor find out the range it covers on this axis (if any)
        let mut sensor_xranges: Vec<(i32, i32)> = sensors
            .iter()
            .filter_map(|sensor| get_xrange(sensor, i))
            .collect();
        // Sort and filter contained ranges
        sensor_xranges.sort_by(|&a, &b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        remove_contained_ranges(&mut sensor_xranges);

        for j in 1..sensor_xranges.len() {
            if (sensor_xranges[j].0 - sensor_xranges[j - 1].1) > 1 {
                // Found a gap to previous range
                let possible_beacon = (sensor_xranges[j - 1].1 + 1, i as i32);
                if possible_beacons.contains(&possible_beacon) {
                    return (possible_beacon.0 as i64) * 4000000_i64 + (possible_beacon.1 as i64);
                }
                possible_beacons.insert(possible_beacon);
            }
        }
    }

    panic!("Did not find the beacon");
    // 13171855019123
}

fn with_index<T, F>(mut f: F) -> impl FnMut(&T) -> bool
where
    F: FnMut(usize, &T) -> bool,
{
    let mut i = 0;
    move |item| (f(i, item), i += 1).0
}

fn remove_contained_ranges(ranges: &mut Vec<(i32, i32)>) {
    // Assume the ranges are sorted on first lower, then upper boundary,
    // filter out all ranges which are partially/fully contained in the previous range.
    // Honestly not sure if this is 100% correct, but it seems to work \o/
    let mut last_upper_bound = i32::MIN;
    ranges.retain(with_index(|index, range: &(i32, i32)| {
        if index == 0 || range.1 > last_upper_bound {
            last_upper_bound = range.1;
            true
        } else {
            false
        }
    }));
}

fn get_yrange(sensor: &(i32, i32, i32), x_axis: i32) -> Option<(i32, i32)> {
    let (sensor_x, sensor_y, radius) = sensor;

    let delta = radius - (x_axis - sensor_x).abs();
    if delta.is_negative() {
        return None;
    }
    Some((sensor_y - delta, sensor_y + delta))
}

fn get_xrange(sensor: &(i32, i32, i32), y: i32) -> Option<(i32, i32)> {
    let (sensor_x, sensor_y, radius) = sensor;

    let delta = radius - (y - sensor_y).abs();
    if delta.is_negative() {
        return None;
    }
    Some((sensor_x - delta, sensor_x + delta))
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

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 56000011);
    }

    #[test]
    fn test_xrange_1() {
        let sensor = (5, 5, 2);
        assert_eq!(get_xrange(&sensor, 9), None);
        assert_eq!(get_xrange(&sensor, 8), None);
        assert_eq!(get_xrange(&sensor, 7), Some((5, 5)));
        assert_eq!(get_xrange(&sensor, 6), Some((4, 6)));
        assert_eq!(get_xrange(&sensor, 5), Some((3, 7)));
        assert_eq!(get_xrange(&sensor, 4), Some((4, 6)));
        assert_eq!(get_xrange(&sensor, 3), Some((5, 5)));
        assert_eq!(get_xrange(&sensor, 2), None);
        assert_eq!(get_xrange(&sensor, -1), None);
    }
}
