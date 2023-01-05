use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    str::Chars,
};

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Cell>>,
    path: Vec<(i32, Option<Dir>)>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut parts = input.split("\n\n");

        let map = parts.next().unwrap();

        let grid: Vec<Vec<Cell>> = map
            .lines()
            .map(|line| line.chars().map(Cell::from).collect_vec())
            .collect();

        let path = parts.next().unwrap().split_inclusive(&['L', 'R']);
        println!("{:?}", path.clone().collect_vec());
        let mut dir = Dir::Right;
        let path = path
            .map(|inst| {
                let len = inst.len();
                if ['L', 'R'].contains(&inst.chars().nth(len-1).unwrap()) {
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

        Self {
            grid,
            path,
        }
    }

    /// Find the starting point on the map. (y, x)
    fn start_pos(&self) -> (usize, usize) {
        (
            0,
            self.grid[0].iter().position(|n| n == &Cell::Open).unwrap(),
        )
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
    let map = Map::new(input);

    println!("{map:#?}",);
    println!("start: {:?}", map.start_pos());
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
