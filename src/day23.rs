use aoc_runner_derive::aoc;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(PartialEq, Eq)]
struct Elf {
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

struct BoundingBox {
    width: i32,
    height: i32,
    origin: Point,
}

struct Grove {
    grid: HashMap<Point, Elf>,
}

impl Grove {
    fn new(grid: HashMap<Point, Elf>) -> Self {
        Self { grid }
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
        todo!();
    }

    fn bounding_box(&self) -> BoundingBox {
        let mut minx = i32::MAX;
        let mut maxx = i32::MIN;
        let mut miny = i32::MAX;
        let mut maxy = i32::MIN;
        for point in self.grid.keys() {
            minx = minx.min(point.0);
            maxx = maxx.max(point.0);
            miny = miny.min(point.1);
            maxy = maxy.max(point.1);
        }

        BoundingBox {
            width: maxx - minx,
            height: maxy - miny,
            origin: Point(minx, miny),
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

impl Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = self.bounding_box();
        for y in 0..=bounds.height {
            for x in 0..=bounds.width {
                let cell = self.grid.get(&Point(x, y));
                let symbol = match cell {
                    Some(_) => '#',
                    None => '.',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1_solve(input: &str) -> usize {
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((Point(x as i32, y as i32), Elf { proposal: None }))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<HashMap<Point, Elf>>();

    let mut grove = Grove::new(grid);

    println!("{}", grove);

    // let area = grove.grid.len() * grove.grid[0].len();

    // (area as usize) - grove.elf_count
    todo!();
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
