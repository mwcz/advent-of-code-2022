use std::collections::HashMap;

use aoc_runner_derive::aoc;

enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[aoc(day9, part1)]
fn part1_solve(input: &str) -> usize {
    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;

    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

    // +y is up

    for line in input.lines() {
        let mut spl = line.split_whitespace();
        let dir = spl.next().unwrap();
        let amt = spl.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..amt {
            match dir {
                "U" => hy += 1,
                "D" => hy -= 1,
                "R" => hx += 1,
                "L" => hx -= 1,
                _ => {}
            }

            match (hx - tx, hy - ty) {
                (2, _) => {
                    // H on right, moving right
                    tx += 1;
                    ty = hy;
                }
                (-2, _) => {
                    // H on right, moving right
                    tx -= 1;
                    ty = hy;
                }
                (_, 2) => {
                    // H above, moving up
                    ty += 1;
                    tx = hx;
                }
                (_, -2) => {
                    ty -= 1;
                    tx = hx;
                }
                _ => {}
            }

            visited.insert((tx, ty), true);
        }
    }

    visited.len()
}

#[aoc(day9, part2)]
fn part2_solve(input: &str) -> usize {
    let mut segs: [(i32, i32); 10] = [(0, 0); 10];

    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

    // +y is up

    for line in input.lines() {
        let mut spl = line.split_whitespace();
        let dir = spl.next().unwrap();
        let amt = spl.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..amt {
            {
                let head = segs.first_mut().unwrap();
                match dir {
                    "U" => head.1 += 1,
                    "D" => head.1 -= 1,
                    "R" => head.0 += 1,
                    "L" => head.0 -= 1,
                    _ => {}
                }
            }

            for i in 0..(segs.len() - 1) {
                let head = segs.get(i).unwrap().clone();
                let tail = segs.get_mut(i + 1).unwrap();

                match (head.0 - tail.0, head.1 - tail.1) {
                    (2, 2) => {
                        tail.0 += 1;
                        tail.1 += 1;
                    }
                    (-2, -2) => {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    }
                    (2, -2) => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    }
                    (-2, 2) => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    }
                    (2, _) => {
                        // H on right, moving right
                        tail.0 += 1;
                        tail.1 = head.1;
                    }
                    (-2, _) => {
                        // H on right, moving right
                        tail.0 -= 1;
                        tail.1 = head.1;
                    }
                    (_, 2) => {
                        // H above, moving up
                        tail.1 += 1;
                        tail.0 = head.0;
                    }
                    (_, -2) => {
                        tail.1 -= 1;
                        tail.0 = head.0;
                    }
                    _ => {}
                }
            }

            visited.insert(segs[segs.len() - 1], true);
        }
    }

    visited.len()
}

#[test]
fn day9_part1_test() {
    let ex = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    assert_eq!(part1_solve(ex), 13);
}

#[test]
fn day9_part2_test() {
    let ex = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    assert_eq!(part2_solve(ex), 36);
}
