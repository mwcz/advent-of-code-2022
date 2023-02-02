use aoc_runner_derive::aoc;
#[cfg(feature = "visualize")]
use console_engine::{ConsoleEngine, KeyCode};
use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::fmt::Display;
use pathfinding::directed::astar::astar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AddAssign, Add, Sub, SubAssign)]
struct Point(i32, i32);

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
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
            let back_to_start = from != self.start && is_start;
            let tl_wall = loc.0 == 0 || loc.1 == 0;
            let br_wall = loc.0 == (self.width - 1) || loc.1 == (self.height - 1);
            let oob = loc.0 < 0 || loc.1 < 0 || loc.0 > self.width || loc.1 > self.height;
            let is_blizz = self.nogo.get( loc.1 as usize ).map_or(true, |row| *row.get(loc.0 as usize).unwrap_or(&true));

            !back_to_start && !is_blizz && !oob && ((is_start || is_end) || (!tl_wall && !br_wall))
        })
        .collect()
    }
}

impl Display for Basin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "({}, {})", self.width, self.height)?;
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

#[derive(Debug, Eq, PartialEq)]
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

fn part1_solve(input: &str) -> usize {
    let mut basin = Basin::new(input);

    #[cfg(feature = "visualize")]
    let print_grid = |basin: &Basin, engine: &mut ConsoleEngine| {
        engine.wait_frame();
        engine.clear_screen();
        engine.print(0, 0, &format!("{}", basin));
        engine.draw();
    };
    #[cfg(feature = "visualize")]
    let fps = 4;
    #[cfg(feature = "visualize")]
    let mut engine = ConsoleEngine::init(basin.width as u32, basin.height as u32 + 1, fps).unwrap();

    let mut paths = vec![(0, vec![basin.start])];

    // rolling progress threshold
    let mut threshold = 0;

    loop {
        // println!("loop");
        basin.step();

        #[cfg(feature = "visualize")]
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        #[cfg(feature = "visualize")]
        print_grid(&basin, &mut engine);

        let path_search: Vec<(i32, Vec<Point>)> = paths.drain(..).collect();

        for (progress, path) in path_search {
            // add each valid move direction to the queue
            for mov in basin.moves(*path.last().unwrap()) {
                let mut updated_path = path.clone();
                updated_path.push(mov);

                let ratio = basin.width / basin.height;

                // calculate how much progress this path is making overall
                let new_progress = path
                    .iter()
                    .map(|p| p.0 + p.1 * ratio)
                    .reduce(|acc, p| p - acc)
                    .unwrap();

                // // calculate how much progress this path has made _recently_
                // let prog_cand = 16; // how many elements to consider for progress calculations
                // let new_progress = path[path.len().checked_sub(prog_cand).unwrap_or(0)..]
                //     .iter()
                //     .map(|p| p.0 + p.1 * ratio)
                //     .reduce(|acc, p| p - acc)
                //     .unwrap();

                if mov == basin.end {
                    return updated_path.len() - 1;
                }

                // if updated_path.len() < 20 || progress >= (updated_path.len()/8) as i32 {
                println!(
                    "len {}\tprogress {}\tthreshold {threshold}\t{}",
                    updated_path.len(),
                    progress,
                    updated_path.iter().map(|p| p.to_string()).collect::<String>()
                );
                if progress >= threshold {
                    // println!("len {}\tprogress {}\tthreshold {threshold}", updated_path.len(), progress);
                    paths.push((new_progress, updated_path));
                }
                // paths.push((new_progress, updated_path));
            }
        }

        // median
        let mut progs: Vec<i32> = paths.iter().map(|(progress, _)| *progress).collect();
        progs.sort();
        if let Some(quartile) = progs.get(progs.len() / 2) {
            threshold = *quartile;
        }

        // // average
        // if let Some(progresses) = paths.iter().map(|(progress, _)| *progress).reduce(|acc, p| acc + p) {
        //     threshold = progresses / (paths.len() as i32);
        // }
    }

    unreachable!();
}

#[aoc(day24, part1)]
fn part1_solver(input: &str) -> usize {
    part1_solve(input)
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
        assert_eq!(part1_solve(REAL), 18);
    }
}