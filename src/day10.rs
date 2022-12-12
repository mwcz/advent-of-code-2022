use std::fmt::Write as _;
use std::str::Lines;

use aoc_runner_derive::aoc;

struct Device<'input> {
    input: Lines<'input>,
    pending: i32,
    x: i32,
    cycle: i32,
}

impl<'input> Device<'input> {
    fn new(input: Lines<'input>) -> Self {
        Self {
            input,
            pending: 0,
            x: 1,
            cycle: 0,
        }
    }
}

impl Iterator for Device<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;
        let strength = self.x * self.cycle;

        // use this cycle to either: apply a pending addx, OR fetch the next command
        if self.pending == 0 {
            let mut tokens = self.input.next().unwrap_or("noop").split_whitespace();
            let cmd = tokens.next().unwrap_or("noop");
            if let "addx" = cmd {
                self.pending = tokens.next().unwrap().parse::<i32>().unwrap_or(0);
            }
        } else {
            self.x += self.pending;
            self.pending = 0;
        }
        Some((self.x, strength))
    }
}

#[aoc(day10, part1)]
fn part1_solve(input: &str) -> i32 {
    let mut dev = Device::new(input.lines());

    [20, 40, 40, 40, 40, 40]
        .iter()
        .filter_map(|n| dev.nth(*n - 1).map(|(_, strength)| strength))
        .sum()
}

#[aoc(day10, part2)]
fn part2_solve(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut dev = Device::new(input.lines());
    let mut answer = String::from("\n");

    // draw 6 rows
    for _ in 1..=6 {
        // draw 40 columns
        for beam in 1..=40 {
            if (beam - dev.next().unwrap().0).abs() <= 1 {
                write!(answer, "#")?;
            } else {
                write!(answer, ".")?;
            }
        }
        writeln!(answer)?;
    }

    Ok(answer)
}

// #[aoc(day10, part2)]
// fn part2_solve(input: &str) -> usize {}

#[test]
fn day10_part1_small_test() {
    let ex = "noop
addx 3
addx -5";
    let mut dev = Device::new(ex.lines());

    assert_eq!(dev.next().unwrap(), (1, 1 * 1));
    assert_eq!(dev.next().unwrap(), (1, 2 * 1));
    assert_eq!(dev.next().unwrap(), (1, 3 * 1));
    assert_eq!(dev.next().unwrap(), (4, 4 * 4));
    assert_eq!(dev.next().unwrap(), (4, 5 * 4));
    assert_eq!(dev.next().unwrap(), (-1, 6 * -1));
    assert_eq!(dev.next().unwrap(), (-1, 7 * -1));
    assert_eq!(dev.next().unwrap(), (-1, 8 * -1));
}

#[test]
fn day10_part1_test() {
    let ex = "addx 15
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
noop";
    let mut dev = Device::new(ex.lines());

    assert_eq!(part1_solve(ex), 13140);
}
