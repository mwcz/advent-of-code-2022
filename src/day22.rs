use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    ops::Add,
    str::Chars,
};

type Step = (i32, Dir);

#[derive(Debug)]
struct Steps(Vec<Step>);

impl Steps {
    fn new(path_str: &str) -> Self {
        let path = path_str.split_inclusive(&['L', 'R']);
        let mut dir = Dir::Right;
        let mut steps = vec![];
        let first = path.next().unwrap();
        // TODO instead of (dist, Option<dir>), continue converting this to (Dir, dist)
        let steps = path
            .map(|inst| {
                let len = inst.len();
                if ['L', 'R'].contains(&inst.chars().nth(len - 1).unwrap()) {
                    let num_part = &inst[..inst.len() - 1];
                    let steps = num_part.parse::<i32>().unwrap();
                    dir = dir.turn(&inst[inst.len() - 1..inst.len()]);
                    (steps, Some(dir))
                } else {
                    let steps = inst.parse::<i32>().unwrap();
                    (steps, None)
                }
            })
            .collect_vec();
        Self(steps)
    }
}

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

type Pos = (Point, Dir);

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Cell>>,
}

impl Map {
    fn new(map_str: &str) -> Self {
        let grid: Vec<Vec<Cell>> = map_str
            .lines()
            .map(|line| line.chars().map(Cell::from).collect_vec())
            .collect();

        Self { grid }
    }

    /// Find the starting point on the map. (x, y)
    fn start_pos(&self) -> Pos {
        (
            Point(
                self.grid[0].iter().position(|n| n == &Cell::Open).unwrap(),
                0,
            ),
            Dir::Right,
        )
    }

    fn step(&self, cur: &Pos, step: &Step) -> Point {
        println!("walk from {cur:?} to {step:?}",);

        let dist = step.0;
        let dir = step.1.unwrap_or(cur.1);
        let mut pos = cur.0;

        for n in 1..=dist {
            let maybe = match dir {
                Dir::Up => Point(pos.0, pos.1),
                Dir::Right => todo!(),
                Dir::Down => todo!(),
                Dir::Left => todo!(),
            };
            println!("{n:?}");
        }

        pos
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Open,
    Wall,
    Void,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn(&self, lr: &str) -> Self {
        use Dir::*;
        match self {
            Up => {
                if lr == "L" {
                    Left
                } else {
                    Right
                }
            }
            Right => {
                if lr == "L" {
                    Up
                } else {
                    Down
                }
            }
            Down => {
                if lr == "L" {
                    Right
                } else {
                    Left
                }
            }
            Left => {
                if lr == "L" {
                    Down
                } else {
                    Up
                }
            }
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Open,
            '#' => Cell::Wall,
            _ => Cell::Void,
        }
    }
}

fn part1_solve(input: &str) -> i32 {
    let mut parts = input.split("\n\n");

    let map = Map::new(parts.next().unwrap());
    let steps = Steps::new(parts.next().unwrap());

    // println!("{map:#?}",);
    // println!("{steps:?}",);
    let mut pos = map.start_pos();
    println!("start: {pos:?}");

    for step in steps.0 {
        pos = map.step(&pos, &step);
    }

    0
}

#[aoc(day22, part1)]
fn part1_solver(input: &str) -> i32 {
    part1_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "        ...#    \n\
                              .#..    \n\
                              #...    \n\
                              ....    \n\
                      ...#.......#    \n\
                      ........#...    \n\
                      ..#....#....    \n\
                      ..........#.    \n\
                              ...#....\n\
                              .....#..\n\
                              .#......\n\
                              ......#.\n\
\n\
                      10R5L5R10L4R5L5";

    #[test]
    fn day22_part1_test() {
        assert_eq!(part1_solve(EX), 152);
    }

    // #[test]
    // fn day22_part2_test() {
    //     assert_eq!(part2_solve(EX), 301);
    // }
}
