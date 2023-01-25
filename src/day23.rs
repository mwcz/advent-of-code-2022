use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(PartialEq, Eq)]
struct Elf {
    loc: Point,
    proposal: Option<Point>,
}

struct Scan {
    n: Point,
    ne: Point,
    e: Point,
    se: Point,
    s: Point,
    sw: Point,
    w: Point,
    nw: Point,
}

struct Grove {
    grid: VecDeque<VecDeque<Cell>>,
    elf_count: usize,
}

impl Grove {
    fn new(grid: VecDeque<VecDeque<Cell>>) -> Self {
        let elf_count = grid.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |row_acc, cell| {
                row_acc + cell.value()
            })
        });
        Self { grid, elf_count }
    }

    fn scan(&self, loc: &Point) -> Scan {
        let n = Point(loc.0, loc.1 - 1);
        let ne = Point(loc.0 + 1, loc.1 - 1);
        let e = Point(loc.0 + 1, loc.1);
        let se = Point(loc.0 + 1, loc.1 + 1);
        let s = Point(loc.0, loc.1 + 1);
        let sw = Point(loc.0 - 1, loc.1 + 1);
        let w = Point(loc.0 - 1, loc.1);
        let nw = Point(loc.0 - 1, loc.1 - 1);

        // TODO resume here: consider using a HashMap<(x,y), Cell> instead of VecDeque<VecDeque<Cell>>
        // it would have no cost of expanding the area
    }
}

#[derive(PartialEq, Eq)]
enum Cell {
    Elf(Elf),
    Empty,
}

impl Cell {
    fn value(&self) -> usize {
        match self {
            Cell::Elf(_) => 1,
            Cell::Empty => 0,
        }
    }
}

impl From<&Cell> for char {
    fn from(val: &Cell) -> Self {
        match val {
            Cell::Elf(_) => '#',
            Cell::Empty => '.',
        }
    }
}

impl Iterator for Grove {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        // first half

        // second half

        Some(())
    }
}

impl ToString for Grove {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().map(|cell| char::from(cell)).collect::<String>())
            .collect_vec()
            .join("\n")
    }
}

fn part1_solve(input: &str) -> usize {
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                // .filter(|(_, c)| c == &'#')
                .map(move |(x, c)| {
                    if c == '#' {
                        Cell::Elf(Elf {
                            loc: Point(x as i32, y as i32),
                            proposal: None,
                        })
                    } else {
                        Cell::Empty
                    }
                })
                .collect::<VecDeque<Cell>>()
        })
        .collect::<VecDeque<VecDeque<Cell>>>();

    let mut grove = Grove::new(grid);

    println!("{}", grove.to_string());
    println!("elves: {}", grove.elf_count);

    let area = grove.grid.len() * grove.grid[0].len();

    (area as usize) - grove.elf_count
}

#[aoc(day23, part1)]
fn part1_solver(input: &str) -> usize {
    part1_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL: &str = include_str!("../input/2022/day23.txt");
    const EX: &str = ".....\n\
                      ..##.\n\
                      ..#..\n\
                      .....\n\
                      ..##.\n\
                      .....";

    const EX2: &str = "....#..\n\
                       ..###.#\n\
                       #...#.#\n\
                       .#...##\n\
                       #.###..\n\
                       ##.#.##\n\
                       .#..#..";

    #[test]
    fn day23_part1_test() {
        assert_eq!(part1_solve(EX2), 110);
    }
}
