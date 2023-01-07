use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{
    character::complete::{self, one_of},
    combinator::{all_consuming, map},
    multi::many1,
    sequence::pair,
    IResult,
};
use std::ops::Add;

type Step = (Dir, i32);

#[derive(Debug)]
struct Steps(Vec<Step>);

impl Steps {
    fn new(path_str: &str) -> Self {
        fn parse(moves: &str) -> IResult<&str, Vec<Step>> {
            let mut dir = Dir::Up; // not to worry; first step will turn this to the right
            let steps = all_consuming(many1(map(
                pair(one_of("LR"), complete::i32),
                |pair: (char, i32)| {
                    dir = dir.turn(pair.0);
                    (dir.clone(), pair.1)
                },
            )))(moves);
            steps
        }

        // prefix with R just to make it more convenient to parse in pairs
        let prefixed = format!("R{}", path_str.trim());
        let (_, steps) = parse(&prefixed).unwrap();

        Steps(steps)
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
    /// The min and max bounds where non-void cells lie.  Used for wrapping around when stepping
    /// into the void.  (row_bounds, col_bounds)
    bounds: (Vec<(usize, usize)>, Vec<(usize, usize)>),
}

impl Map {
    fn new(map_str: &str) -> Self {
        let mut grid: Vec<Vec<Cell>> = map_str
            .lines()
            .map(|line| line.chars().map(Cell::from).collect_vec())
            .collect();

        // fill in empty cells (since the lines in the input aren't all the same number of
        // characters 🤦

        let max_row_len = grid.iter().map(|r| r.len()).max().unwrap();
        for row in grid.iter_mut() {
            row.resize_with(max_row_len, || Cell::Void);
        }

        // calculate bounds

        let mut bounds = (vec![], vec![]);
        for row in &grid {
            let min_bound = row.iter().position(|n| n != &Cell::Void).unwrap();
            let max_bound = row
                .iter()
                .enumerate()
                .rev()
                .find(|n| n.1 != &Cell::Void)
                .unwrap()
                .0;
            bounds.0.push((min_bound, max_bound));
        }
        for col_idx in 0..grid[0].len() {
            let mut min_bound = grid.len();
            let mut max_bound = 0;

            for row_idx in 0..grid.len() {
                let cell = &grid[row_idx][col_idx];
                if cell != &Cell::Void {
                    min_bound = row_idx;
                    break;
                }
            }

            for row_idx in (0..grid.len()).rev() {
                let cell = &grid[row_idx][col_idx];
                if cell != &Cell::Void {
                    max_bound = row_idx;
                    break;
                }
            }

            bounds.1.push((min_bound, max_bound));
        }

        Self { grid, bounds }
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

    fn step(&self, cur: &Point, step: &Step) -> Point {
        // print!("walk {step:?} start {cur:?}",);

        let mut cur = *cur;
        for _i in 1..=step.1 {
            // println!("  take step {i} of {} to the {:?}", step.1, step.0);
            cur = self.next_point(&cur, &step.0);
        }

        // println!(" end {cur:?}");

        cur
    }

    fn next_point(&self, cur: &Point, dir: &Dir) -> Point {
        return match dir {
            Dir::Up => {
                let up_y = (self.grid.len() + cur.1 - 1) % self.grid.len();
                match self.grid[up_y][cur.0] {
                    Cell::Open => Point(cur.0, up_y),
                    Cell::Wall => *cur,
                    Cell::Void => {
                        let p = Point(cur.0, self.bounds.1[cur.0].1);
                        if self.grid[p.1][p.0] == Cell::Open {
                            p
                        } else {
                            *cur
                        }
                    }
                }
            }
            Dir::Right => {
                let right_x = (cur.0 + 1) % self.grid[0].len();
                match self.grid[cur.1][right_x] {
                    Cell::Open => Point(right_x, cur.1),
                    Cell::Wall => *cur,
                    Cell::Void => {
                        let p = Point(self.bounds.0[cur.1].0, cur.1);
                        if self.grid[p.1][p.0] == Cell::Open {
                            p
                        } else {
                            *cur
                        }
                    }
                }
            }
            Dir::Down => {
                let down_y = (cur.1 + 1) % self.grid.len();
                match self.grid[down_y][cur.0] {
                    Cell::Open => Point(cur.0, down_y),
                    Cell::Wall => *cur,
                    Cell::Void => {
                        let p = Point(cur.0, self.bounds.1[cur.0].0);
                        if self.grid[p.1][p.0] == Cell::Open {
                            p
                        } else {
                            *cur
                        }
                    }
                }
            }
            Dir::Left => {
                let left_x = (self.grid[0].len() + cur.0 - 1) % self.grid[0].len();
                match self.grid[cur.1][left_x] {
                    Cell::Open => Point(left_x, cur.1),
                    Cell::Wall => *cur,
                    Cell::Void => {
                        let p = Point(self.bounds.0[cur.1].1, cur.1);
                        if self.grid[p.1][p.0] == Cell::Open {
                            p
                        } else {
                            *cur
                        }
                    }
                }
            }
        };
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
    fn turn(&self, lr: char) -> Self {
        use Dir::*;
        match self {
            Up => {
                if lr == 'L' {
                    Left
                } else {
                    Right
                }
            }
            Right => {
                if lr == 'L' {
                    Up
                } else {
                    Down
                }
            }
            Down => {
                if lr == 'L' {
                    Right
                } else {
                    Left
                }
            }
            Left => {
                if lr == 'L' {
                    Down
                } else {
                    Up
                }
            }
        }
    }
    fn score(&self) -> usize {
        match self {
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Right => 0,
            Dir::Up => 3,
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

fn part1_solve(input: &str) -> usize {
    let mut parts = input.split("\n\n");

    let map = Map::new(parts.next().unwrap());
    let steps = Steps::new(parts.next().unwrap());

    // println!("{map:#?}",);
    // println!("{steps:?}",);
    let (mut pos, mut dir) = map.start_pos();
    // println!("start: {pos:?}");

    for step in &steps.0 {
        pos = map.step(&pos, &step);
        dir = step.0;
    }

    // println!("Final pos: {pos:?}",);
    // println!("Final dir: {dir:?}",);

    // println!("{}", steps.0.len());

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir.score()
}

#[aoc(day22, part1)]
fn part1_solver(input: &str) -> usize {
    part1_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL: &str = include_str!("../input/2022/day22.txt");
    const EX: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    const EX2: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

1R0";
    const EX3: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

1L1L1L1L0";

    #[test]
    fn day22_part1_test() {
        assert_eq!(part1_solve(EX), 6032);
        assert_eq!(part1_solve(EX2), 1041);
        assert_eq!(part1_solve(EX3), 1036);
        assert_eq!(part1_solve(REAL), 146092);
    }

    // #[test]
    // fn day22_part2_test() {
    //     assert_eq!(part2_solve(EX), 301);
    // }
}
