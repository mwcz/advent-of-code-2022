use aoc_runner_derive::aoc;
use console_engine::{KeyCode, ConsoleEngine};
use itertools::Itertools;
use nom::{
    character::complete::{self, one_of},
    combinator::{all_consuming, map},
    multi::many1,
    sequence::pair,
    IResult,
};
use std::{collections::HashMap, ops::Add};

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
                    (dir, pair.1)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}

type Pos = (Point, Dir);

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Cell>>,
    /// The min and max bounds where non-void cells lie.  Used for wrapping around when stepping
    /// into the void.  (row_bounds, col_bounds)
    bounds: (Vec<Point>, Vec<Point>),
    net_portals: HashMap<Point, Point>,
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

        // calculate bounds (used in part 1)

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
            bounds.0.push(Point(min_bound, max_bound));
        }
        for col_idx in 0..grid[0].len() {
            let mut min_bound = grid.len();
            let mut max_bound = 0;

            for (row_idx, row) in grid.iter().enumerate() {
                let cell = &row[col_idx];
                if cell != &Cell::Void {
                    min_bound = row_idx;
                    break;
                }
            }

            for (row_idx, row) in grid.iter().enumerate().rev() {
                let cell = &row[col_idx];
                if cell != &Cell::Void {
                    max_bound = row_idx;
                    break;
                }
            }

            bounds.1.push(Point(min_bound, max_bound));
        }

        use Dir::*;
        // Find a concave right angle in the net, to be a starting point for zipping the cube back
        // together.  Returns the point where the nook is and the directions of each line coming out
        // of it.
        // TODO find this dynamically
        let seam_start = (Point(8, 4), Dir::Left, Dir::Up); // for the example

        // let seam_start = (Point(50, 100), Dir::Left, Dir::Up); // for the real input

        let mut net_portals = HashMap::new();

        /*

                ...#
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

                */

        // zip the seam back together

        let cell = |x: usize, x_off: isize, y: usize, y_off: isize| -> &Cell {
            let Some(new_x) = x.checked_add_signed(x_off) else { 
                return &Cell::Void;
            };
            let Some(new_y) = y.checked_add_signed(y_off) else {
                return &Cell::Void;
            };
            if let Some(row) = grid.get(new_y) {
                row.get(new_x).unwrap_or(&Cell::Void)
            } else {
                &Cell::Void
            }
        };

        type Kernel<'a> = [[&'a Cell; 3]; 3];

        #[rustfmt::skip]
        let kernel = |p: Point| -> Kernel {[
            [cell(p.0, -1, p.1, -1), cell(p.0, 0, p.1, -1), cell(p.0, 1, p.1, -1)],
            [cell(p.0, -1, p.1,  0), cell(p.0, 0, p.1,  0), cell(p.0, 1, p.1,  0)],
            [cell(p.0, -1, p.1,  1), cell(p.0, 0, p.1,  1), cell(p.0, 1, p.1,  1)],
        ]};

        // Some(p) if the point at the center of a kernel is where a turn should take place because
        // it's a corner, either concave or convex, along the cube net's seam. If no turn should
        // take place, returns None.
        let corner = |k: Kernel| -> Option<(Dir, Dir)> {
            // a kernel contains a corner if one of the following is true:
            //    a: if there are 5 voids in in the kernel, center is a convex corner
            //    b: if there is 1 void in the kernel, center is a concave corner
            use Dir::*;

            let corners = [
                (k[0][0], (Left, Up)),
                (k[0][2], (Right, Up)),
                (k[2][0], (Left, Down)),
                (k[2][2], (Right, Down)),
            ];

            let voids = k.iter().flatten().filter(|&p| *p == &Cell::Void).count();

            match voids {
                // concave corner
                1 => Some(corners.iter().find(|(p, _)| *p == &Cell::Void).unwrap().1),
                // convex corner
                5 => Some(corners.iter().find(|(p, _)| *p != &Cell::Void).unwrap().1),
                // no corner
                _ => None,
            }
        };
        // TODO resume here, test out corner()

        let zippers = || -> Vec<(Point, (Dir, Dir))> {
            let mut points = vec![];
            for y in 1..(grid.len()-1) {
                for x in 1..(grid[0].len()-1) {
                    let k = kernel(Point(x, y));
                    if let Some(c) = corner(k) {
                        points.push( (Point(x,y), c) );
                    }
                }
            }
            points
        };

        let start_points = zippers();

        let mut dirs;
        let mut points;
        let mut turning;

        let width: u32 = (map_str.lines().next().unwrap().len() + 4).try_into().unwrap();
        let height: u32 = (map_str.lines().collect_vec().len() + 5).try_into().unwrap();
        let fps: u32 = 5;
        let mut engine = ConsoleEngine::init(width, height, fps).unwrap();

        let print_grid = |i: i32, points: &[Point; 2],dirs: &[Dir; 2], net_portals: &HashMap<Point, Point>, engine: &mut ConsoleEngine| {
            engine.wait_frame();
            engine.clear_screen();

            engine.print(0, (height as i32)-4, &format!("iteration {}", i));
            engine.print(0, (height as i32)-3, "█ - portal");
            engine.print(0, (height as i32)-2, "X - concave corner");
            engine.print(0, (height as i32)-1, "<v^> - agent travel direction");
            let starts: Vec<String> = start_points.iter().map(|p| format!("{} {} {}", p.0.to_string(), p.1.0.to_string(), p.1.1.to_string())).collect();
            engine.print(0, (height as i32), &format!("{}", starts.join(" / ")));

            for (y, row) in grid.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let p = Point(x, y);
                    let x = x as i32;
                    let y = y as i32;
                    if let Some(portal) = net_portals.get(&p) {
                        engine.print(x, y, "█");
                        // print!("O");
                    } else if start_points.iter().find(|c| c.0 == p).is_some() {
                        engine.print(x, y, "X");
                    } else if points[0] == p {
                        engine.print(x, y, dirs[0].into());
                    } else if points[1] == p {
                        engine.print(x, y, dirs[1].into());
                    } else {
                        engine.print(x, y, cell.into());
                    }
                }
            }

            engine.draw();
        };

        let mut i = 0;
        for start in &start_points {
            let mut ii = 0;

            dirs = [start.1.0, start.1.1];
            points = [start.0 + dirs[0], start.0 + dirs[1]];
            turning = [false, false];

            loop {
                // turning occupies one iteration, since the corner is attached to two other points
                i += 1;
                ii += 1;

                if engine.is_key_pressed(KeyCode::Char('q')) {
                    break;
                }


                // check for a turn
                let kernels: [Kernel; 2] = points.map(kernel);

                let corners: [Option<(Dir, Dir)>; 2] = kernels.map(corner);

                // TODO if _both_ points turn at the same time, this algorithm needs to be restarted at
                // another one of the starting nooks

                print_grid(i, &points, &dirs, &net_portals, &mut engine);

                net_portals.insert(points[0], points[1]);
                net_portals.insert(points[1], points[0]);

                // either turn or move
                for i in 0..=1 {
                    if turning[i] {
                        turning[i] = false;
                    } else {
                        if let Some(corner) = corners[i] {
                            dirs[i] = match dirs[i] {
                                Up => corner.0,
                                Right => corner.1,
                                Down => corner.0,
                                Left => corner.1,
                            };
                            turning[i] = true;
                        }
                        points[i] = points[i] + dirs[i];
                    }
                }

                if turning[0] && turning[1] {
                    break;
                }

                if ii > 1 && points[0] == points[1] {
                    // reached the end of the seam
                    break;
                }

            }
        }

        Self {
            grid,
            bounds,
            net_portals,
        }
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
            cur = self.next_point(&cur, &step.0);
        }

        cur
    }

    fn next_point(&self, cur: &Point, dir: &Dir) -> Point {
        match dir {
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
        }
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

impl From<Dir> for &str {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => "^",
            Dir::Right => ">",
            Dir::Down => "v",
            Dir::Left => "<",
        }
    }
}

impl ToString for Dir {
    fn to_string(&self) -> String {
        match self {
            Dir::Up => "^",
            Dir::Right => ">",
            Dir::Down => "v",
            Dir::Left => "<",
        }.to_string()
    }
}

impl Dir {
    fn turn(&self, lr: char) -> Self {
        if lr == 'L' {
            self.ccw()
        } else {
            self.cw()
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
    fn cw(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
    fn ccw(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
}

impl Add<Dir> for Point {
    type Output = Point;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::Up => Point(self.0, self.1.checked_sub(1).unwrap()),
            Dir::Right => Point(self.0 + 1, self.1),
            Dir::Down => Point(self.0, self.1 + 1),
            Dir::Left => Point(self.0.checked_sub(1).unwrap(), self.1),
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

impl From<&Cell> for &str {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::Open => ".",
            Cell::Wall => "#",
            Cell::Void => " ",
        }
    }
}


fn part1_solve(input: &str) -> usize {
    let mut parts = input.split("\n\n");

    let map = Map::new(parts.next().unwrap());
    let steps = Steps::new(parts.next().unwrap());

    let (mut pos, mut dir) = map.start_pos();

    for step in &steps.0 {
        pos = map.step(&pos, step);
        dir = step.0;
    }

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir.score()
}

#[aoc(day22, part1)]
fn part1_solver(input: &str) -> usize {
    part1_solve(input)
}

fn part2_solve(input: &str) -> usize {
    let mut parts = input.split("\n\n");

    let map = Map::new(parts.next().unwrap());
    let steps = Steps::new(parts.next().unwrap());

    let (mut pos, mut dir) = map.start_pos();

    for step in &steps.0 {
        pos = map.step(&pos, step);
        dir = step.0;
    }

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir.score()
}

#[aoc(day22, part2)]
fn part2_solver(input: &str) -> usize {
    part2_solve(input)
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

    #[test]
    fn day22_part2_test() {
        assert_eq!(part2_solve(EX), 5031);
    }
    #[test]
    fn day22_part2_real() {
        assert_eq!(part2_solve(REAL), 5031);
    }
}
