use aoc_runner_derive::aoc;
use console_engine::crossterm::style::Stylize;
use derive_more::{Add, AddAssign, Sub, SubAssign};
use itertools::Itertools;
use pathfinding::directed::astar::astar;
use std::fmt::Display;

#[cfg(feature = "visualize")]
use console_engine::{ConsoleEngine, KeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AddAssign, Add, Sub, SubAssign, Hash)]
struct Point(i32, i32);

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
struct Basin {
    blizzards: Vec<Blizz>,
    nogo: Vec<Vec<bool>>,
    height: i32,
    width: i32,
    start: Point,
    end: Point,
}

impl Basin {
    fn new(input: &str) -> Self {
        // parse the input

        let mut start: Option<Point> = None;
        let mut end = Point(0, 0);
        let mut height = 0;
        let mut width = 0;

        let mut blizzards = vec![];

        for (y, line) in input.lines().enumerate() {
            height = height.max(y as i32);
            for (x, c) in line.chars().enumerate() {
                width = width.max(x as i32);
                let cell = Cell::from((c, x as i32, y as i32));
                if let Some(blizz_cell) = match cell {
                    Cell::Empty if start.is_none() => {
                        start = Some(Point(x as i32, y as i32));
                        None
                    }
                    Cell::Empty => {
                        end = Point(x as i32, y as i32);
                        None
                    }
                    Cell::Up(_) => Some(cell),
                    Cell::Down(_) => Some(cell),
                    Cell::West(_) => Some(cell),
                    Cell::East(_) => Some(cell),
                    _ => None,
                } {
                    blizzards.push(Blizz::try_from(&blizz_cell).unwrap());
                }
            }
        }

        Self {
            blizzards,
            nogo: vec![vec![false; width as usize]; height as usize],
            height: height + 1, // +1 for the outer wall
            width: width + 1,   // +1 for the outer wall
            start: start.unwrap(),
            end,
        }
    }

    fn step(&mut self) {
        for row in self.nogo.iter_mut() {
            for cell in row.iter_mut() {
                *cell = false;
            }
        }
        for blizz in self.blizzards.iter_mut() {
            blizz.loc += blizz.dir;
            if blizz.loc.0 == 0 {
                blizz.loc.0 = self.width - 2;
            }
            if blizz.loc.1 == 0 {
                blizz.loc.1 = self.height - 2;
            }
            if blizz.loc.0 == self.width - 1 {
                blizz.loc.0 = 1;
            }
            if blizz.loc.1 == self.height - 1 {
                blizz.loc.1 = 1;
            }
            self.nogo[blizz.loc.1 as usize][blizz.loc.0 as usize] = true;
        }
    }

    /// Find valid moves from a given point at the current timestep.
    fn moves(&self, from: Point) -> Vec<Point> {
        [
            from,                // wait
            from + Point(1, 0),  // right
            from + Point(-1, 0), // left
            from + Point(0, 1),  // down
            from + Point(0, -1), // up
        ]
        .into_iter()
        .filter(|&loc| {
            let is_start = loc == self.start;
            let is_end = loc == self.end;
            // let back_to_start = from != self.start && is_start;
            let tl_wall = loc.0 == 0 || loc.1 == 0;
            let br_wall = loc.0 == (self.width - 1) || loc.1 == (self.height - 1);
            let oob = loc.0 < 0 || loc.1 < 0 || loc.0 > self.width || loc.1 > self.height;
            let is_blizz = self
                .nogo
                .get(loc.1 as usize)
                .map_or(true, |row| *row.get(loc.0 as usize).unwrap_or(&true));

            let is_good = is_end
                || (/* !back_to_start &&*/ !is_blizz && !oob && ((is_start) || !(tl_wall || br_wall)));
            // println!("{loc}\tis_start:\t{is_start}\tis_end:\t{is_end}\tback_to_start:\t{back_to_start}\ttl_wall:\t{tl_wall}\tbr_wall:\t{br_wall}\toob:\t{oob}\tis_blizz:\t{is_blizz}");

            is_good
        })
        .collect()
    }
}

impl Display for Basin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point(x, y);
                let blizz: Vec<&Blizz> = self.blizzards.iter().filter(|&b| b.loc == p).collect();
                if (y == 0 || x == 0 || y == self.height - 1 || x == self.width - 1)
                    && self.start != p
                    && self.end != p
                {
                    write!(f, "#")?;
                } else if blizz.len() > 1 {
                    write!(f, "{}", blizz.len())?;
                } else if blizz.len() == 1 {
                    write!(
                        f,
                        "{}",
                        std::convert::Into::<char>::into(*blizz.first().unwrap())
                    )?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Up(Point),
    Down(Point),
    West(Point),
    East(Point),
}

impl From<(char, i32, i32)> for Cell {
    fn from((c, x, y): (char, i32, i32)) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            '^' => Cell::Up(Point(x, y)),
            'v' => Cell::Down(Point(x, y)),
            '<' => Cell::West(Point(x, y)),
            '>' => Cell::East(Point(x, y)),
            _ => panic!("invalid char {c}"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Blizz {
    loc: Point,
    dir: Point,
}

impl Into<char> for &Blizz {
    fn into(self) -> char {
        match (self.dir.0, self.dir.1) {
            (0, 1) => 'v',
            (0, -1) => '^',
            (1, 0) => '>',
            (-1, 0) => '<',
            _ => 'x',
        }
    }
}

impl TryFrom<&Cell> for Blizz {
    type Error = ();

    fn try_from(value: &Cell) -> Result<Self, Self::Error> {
        match value {
            Cell::Wall => Err(()),
            Cell::Empty => Err(()),
            Cell::Up(p) => Ok(Blizz {
                loc: *p,
                dir: Point(0, -1),
            }),
            Cell::Down(p) => Ok(Blizz {
                loc: *p,
                dir: Point(0, 1),
            }),
            Cell::West(p) => Ok(Blizz {
                loc: *p,
                dir: Point(-1, 0),
            }),
            Cell::East(p) => Ok(Blizz {
                loc: *p,
                dir: Point(1, 0),
            }),
        }
    }
}

fn part1_solve(input: &str) -> i32 {
    let mut basin = Basin::new(input);

    let basins = (0..(basin.width * basin.height))
        .into_iter()
        .map(|_| {
            basin.step();
            basin.clone()
        })
        .collect_vec();

    // for pathfinding, combine the grid Point with a usize representing the iteration number.
    // this is only because the pathfinding crate's algos refuse to revisit the same point twice,
    // but revisiting is required to solve this problem.
    type PathPoint = (Point, usize);
    let start = (basin.start, 0);
    let successors = |p: &PathPoint| -> Vec<(PathPoint, i32)> {
        basins[p.1]
            .moves(p.0)
            .into_iter()
            .map(|next| ((next, p.1 + 1), 1))
            .collect()
    };
    let heuristic = |p: &PathPoint| {
        let diff = basin.end - p.0;
        diff.0 + diff.1
    };
    let success = |p: &PathPoint| p.0 == basin.end;

    let answer = astar(&start, successors, heuristic, success);

    answer.unwrap().1
}

#[aoc(day24, part1)]
fn part1_solver(input: &str) -> i32 {
    part1_solve(input)
}

fn part2_solve(input: &str) -> i32 {
    let mut basin = Basin::new(input);

    #[cfg(feature = "visualize")]
    let print_grid = |basin: &Basin, player: &Point, engine: &mut ConsoleEngine| {
        engine.wait_frame();
        engine.clear_screen();
        let basin_out = format!("{}", basin);
        let basin_out = basin_out
            .lines()
            .enumerate()
            .map(|(y, line)| {
                if y == player.1 as usize {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| if x == player.0 as usize { '█' } else { c })
                        .collect::<String>()
                } else {
                    line.to_string()
                }
            })
            .collect_vec();
        engine.print(0, 0, &basin_out.join("\n"));
        engine.draw();
    };

    #[cfg(feature = "visualize")]
    let fps = 30;
    #[cfg(feature = "visualize")]
    let mut engine = ConsoleEngine::init(basin.width as u32, basin.height as u32 + 1, fps).unwrap();

    let basins = (0..3 * (basin.width * basin.height))
        .into_iter()
        .map(|_| {
            basin.step();
            basin.clone()
        })
        .collect_vec();

    // for pathfinding, combine the grid Point with a usize representing the iteration number.
    // this is only because the pathfinding crate's algos refuse to revisit the same point twice,
    // but revisiting is required to solve this problem.
    type PathPoint = (Point, usize);
    let successors = |p: &PathPoint| -> Vec<(PathPoint, i32)> {
        let moves = basins[p.1]
            .moves(p.0)
            .into_iter()
            .map(|next| ((next, p.1 + 1), 1))
            .collect();
        // println!("{moves:?}");
        moves
    };
    let heuristic1 = |p: &PathPoint| {
        let diff = basin.end - p.0;
        diff.0 + diff.1
    };
    let heuristic2 = |p: &PathPoint| {
        let diff = basin.start - p.0;
        diff.0 + diff.1
    };
    let success1 = |p: &PathPoint| p.0 == basin.end;
    let success2 = |p: &PathPoint| p.0 == basin.start;

    // to goal
    let start1 = (basin.start, 0);
    let phase1 = astar(&start1, successors, heuristic1, success1).unwrap();

    // back to start
    let start2 = (basin.end, phase1.1 as usize);
    let phase2 = astar(&start2, successors, heuristic2, success2).unwrap();

    // back to goal with little elfie mcforgetful's snacks
    let start3 = (basin.start, (phase1.1 + phase2.1) as usize);
    let phase3 = astar(&start3, successors, heuristic1, success1).unwrap();

    #[cfg(feature = "visualize")]
    {
        let steps = phase1
            .0
            .iter()
            .chain(phase2.0.iter())
            .chain(phase3.0.iter());
        let mut last_step = 0;
        for step in steps {
            if engine.is_key_pressed(KeyCode::Char('q')) {
                break;
            }
            print_grid(basins.get(step.1).unwrap(), &step.0, &mut engine);
            last_step = step.1;
        }
        let last_basin = basins.get(last_step).unwrap();

        // keep the animation running for a few more steps
        for _ in 0..14 {
            last_step += 1;
            print_grid(
                &basins.get(last_step).unwrap(),
                &last_basin.end,
                &mut engine,
            );
        }
    }

    phase1.1 + phase2.1 + phase3.1
}

#[aoc(day24, part2)]
fn part2_solver(input: &str) -> i32 {
    part2_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL: &str = include_str!("../input/2022/day24.txt");
    const EX: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn day24_part1_example() {
        assert_eq!(part1_solve(EX), 18);
    }
    #[test]
    fn day24_part1_real() {
        assert_eq!(part1_solve(REAL), 290);
    }

    #[test]
    fn day24_part2_example() {
        assert_eq!(part2_solve(EX), 54);
    }
    #[test]
    fn day24_part2_real() {
        assert_eq!(part2_solve(REAL), 842);
    }
}
