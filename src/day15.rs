use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Record {
    sensor: Point,
    beacon: Point,
    dist: i64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn dist(&self, other: &Point) -> i64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64
    }
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    part1_solve(input, 2000000)
}

fn part1_solve(input: &str, row: i64) -> usize {
    let mut records: Vec<Record> = vec![];

    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;

    for line in input.lines() {
        let line = &line[12..];
        let line = line.split_once(',').unwrap();
        let sensor_x = line.0.parse::<i64>().unwrap();
        let line = &line.1[3..];
        let line = line.split_once(':').unwrap();
        let sensor_y = line.0.parse::<i64>().unwrap();
        let line = &line.1[24..];
        let line = line.split_once(',').unwrap();
        let beacon_x = line.0.parse::<i64>().unwrap();
        let line = &line.1[3..];
        let beacon_y = line.parse::<i64>().unwrap();

        let sensor = Point {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon = Point {
            x: beacon_x,
            y: beacon_y,
        };
        let dist = sensor.dist(&beacon);

        x_min = x_min.min(sensor_x - dist);
        x_max = x_max.max(sensor_x + dist);

        records.push(Record {
            sensor,
            beacon,
            dist,
        });
    }

    let mut no_count = 0;
    'outer: for x in x_min..=x_max {
        let p = Point {
            x,
            y: row,
        };

        // check each record to see if p's dist to the sensor is less than its dist to the beacon,
        // if true then inc no_count

        for record in &records {
            if p.dist(&record.sensor) <= record.dist {
                if !records.iter().any(|r| r.beacon == p) {
                    no_count += 1;
                }
                continue 'outer;
            }
        }
    }

    no_count
}

#[aoc(day15, part2)]
fn part2(input: &str) -> i64 {
    part2_solve(input, 4000000)
}

fn part2_solve(input: &str, size: i64) -> i64 {
    let mut records: Vec<Record> = vec![];

    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;
    let mut y_min = i64::MAX;
    let mut y_max = i64::MIN;

    for line in input.lines() {
        let line = &line[12..];
        let line = line.split_once(',').unwrap();
        let sensor_x = line.0.parse::<i64>().unwrap();
        let line = &line.1[3..];
        let line = line.split_once(':').unwrap();
        let sensor_y = line.0.parse::<i64>().unwrap();
        let line = &line.1[24..];
        let line = line.split_once(',').unwrap();
        let beacon_x = line.0.parse::<i64>().unwrap();
        let line = &line.1[3..];
        let beacon_y = line.parse::<i64>().unwrap();

        let sensor = Point {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon = Point {
            x: beacon_x,
            y: beacon_y,
        };
        let dist = sensor.dist(&beacon);

        x_min = x_min.min(sensor_x - dist);
        x_max = x_max.max(sensor_x + dist);

        y_min = y_min.min(sensor_y - dist);
        y_max = y_max.max(sensor_y + dist);

        records.push(Record {
            sensor,
            beacon,
            dist,
        });
    }

    // check around the perimeter of each sensor's scan area; the answer is guaranteed to lie just
    // outside the perim.

    let dr = (1, 1); // down right
    let dl = (-1, 1); // down left
    let ul = (-1, -1); // up left
    let ur = (1, -1); // up right

    // initial direction, down-right
    let mut dir: (i64, i64);

    let search_range = 0..=size;

    for record in &records {
        let mut point = record.sensor.clone();

        dir = dr;

        // starting with the top corner of the area, move clockwise around the perimeter of the
        // scan area.  when considering each point, make sure it's inside the 0..=area range.

        point.y = point.y - record.dist - 1;

        // loop over every cell in the perimeter
        for _ in 0..(4*(record.dist+1)) {
            point.x += dir.0;
            point.y += dir.1;

            if !(search_range.contains(&point.x) && search_range.contains(&point.y)) {
                // don't search outside the search area
                continue;
            }

            for r in &records {
                if point.dist(&r.sensor) > r.dist {
                }
            }

            if records.iter().all(|r| point.dist(&r.sensor) > r.dist) {
                return point.x * 4000000 + point.y;
            }

            // change direction
            if dir == dr && point.x == record.sensor.x + record.dist + 1 {
                dir = dl;
            } else if dir == dl && point.y == record.sensor.y + record.dist + 1 {
                dir = ul;
            } else if dir == ul && point.x == record.sensor.x - record.dist - 1 {
                dir = ur;
            }
        }

    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solve(EX, 10), 26);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solve(EX, 20), 56000011);
    }
}
