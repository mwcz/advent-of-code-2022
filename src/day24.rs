use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(usize, usize);

#[derive(Debug)]
struct Basin {
    blizzards: Vec<Cell>,
    height: u32,
    width: u32,
    loc: Point,
    start: Point,
    end: Point,
}

impl Basin {
    fn new(input: &str) -> Self {
        // parse the input

        let mut start: Option<Point> = None;
        let mut end = Point(0,0);

        let blizzards = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().filter_map(|(x, c)| {
                    let cell = Cell::from((c, x, y));
                    match cell {
                        Cell::Wall => None,
                        Cell::Empty if start.is_none() => {
                            start = Some(Point(x,y));
                            None
                        }
                        Cell::Empty => {
                            end = Point(x, y);
                            None
                        }
                        Cell::Up(_) => Some(cell),
                        Cell::Down(_) => Some(cell),
                        Cell::West(_) => Some(cell),
                        Cell::East(_) => Some(cell),
                    }
                })
            })
            .flatten()
            .collect_vec();

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

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Up(Point),
    Down(Point),
    West(Point),
    East(Point),
}

impl From<(char, usize, usize)> for Cell {
    fn from((c, x, y): (char, usize, usize)) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            '^' => Cell::Up(Point(x,y)),
            'v' => Cell::Down(Point(x,y)),
            '<' => Cell::West(Point(x,y)),
            '>' => Cell::East(Point(x,y)),
            _ => panic!("invalid char {c}"),
        }
    }
}

fn part1_solve(input: &str) -> i64 {
    let basin = Basin::new(input);
    println!("{basin:#?}",);
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
