use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

struct Elf {
    loc: Point,
    proposal: Option<Point>
}

#[derive(Debug)]
struct BoundingBox {
    min: Point,
    max: Point,
    width: i32,
    height: i32,
}

struct Grove {
    elves: Vec<Elf>,
}

impl Grove {
    /// Return the (min, max) points of an AA bounding box around the elves.
    fn bounding_box(&self) -> BoundingBox {
        let mut min = Point(i32::MAX, i32::MAX);
        let mut max = Point(i32::MIN, i32::MIN);

        for elf in &self.elves {
            min.0 = min.0.min(elf.loc.0);
            min.1 = min.1.min(elf.loc.1);
            max.0 = max.0.max(elf.loc.0);
            max.1 = max.1.max(elf.loc.1);
        }

        let width = max.0 - min.0 + 1;
        let height = max.1 - min.1 + 1;

        BoundingBox {
            min,
            max,
            width,
            height,
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
        let bounds = self.bounding_box();
        let mut grid = vec![".".repeat(bounds.width as usize); bounds.height as usize];

        let offset_x = bounds.min.0;
        let offset_y = bounds.min.1;

        for elf in &self.elves {
            let x = (elf.loc.0 - offset_x) as usize;
            let y = (elf.loc.1 - offset_y) as usize;
            let row = grid.get_mut(y).unwrap();
            row.replace_range(x..x + 1, "#");
        }

        grid.join("\n")
    }
}

fn part1_solve(input: &str) -> usize {
    let mut elves = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, _)| Elf {
                    loc: Point(x as i32, y as i32),
                    proposal: None,
                })
        })
        .flatten()
        .collect_vec();

    let mut grove = Grove { elves };

    println!("{}", grove.to_string());

    let bounds = grove.bounding_box();
    let area = bounds.width * bounds.height;

    (area as usize) - grove.elves.len()
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
