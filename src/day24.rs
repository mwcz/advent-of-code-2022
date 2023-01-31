use aoc_runner_derive::aoc;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(i32, i32);

#[derive(Debug)]
struct Basin {
    blizzards: Vec<Blizz>,
    height: i32,
    width: i32,
    loc: Point,
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

        let loc = start;

        Self {
            blizzards,
            height,
            width,
            loc: loc.unwrap(),
            start: start.unwrap(),
            end,
        }
    }

    fn step(&mut self) {}
}

impl Display for Basin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.height {
            for x in 0..=self.width {
                let p = Point(x, y);
                let blizz = self.blizzards.iter().find(|&b| b.loc == p);
                if y == 0
                    || x == 0
                    || y == self.height
                    || x == self.width && self.start != p && self.end != p
                {
                    write!(f, "#")?;
                } else if blizz.is_some() {
                    write!(f, "{}", std::convert::Into::<char>::into(blizz.unwrap()))?;
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

fn part1_solve(input: &str) -> i64 {
    let basin = Basin::new(input);
    println!("{basin}",);
    todo!();
}

#[aoc(day24, part1)]
fn part1_solver(input: &str) -> i64 {
    part1_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn day24_part1_test() {
        assert_eq!(part1_solve(EX), 18);
    }
}
