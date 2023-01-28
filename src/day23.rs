use aoc_runner_derive::aoc;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(PartialEq, Eq)]
struct Elf {
    proposal: Option<Point>,
}

struct Survey<'elf> {
    n: Option<&'elf Elf>,
    ne: Option<&'elf Elf>,
    e: Option<&'elf Elf>,
    se: Option<&'elf Elf>,
    s: Option<&'elf Elf>,
    sw: Option<&'elf Elf>,
    w: Option<&'elf Elf>,
    nw: Option<&'elf Elf>,
}

impl<'elf> Survey<'elf> {
    fn is_empty(&self) -> bool {
        self.n.is_none()
            && self.ne.is_none()
            && self.e.is_none()
            && self.se.is_none()
            && self.s.is_none()
            && self.sw.is_none()
            && self.w.is_none()
            && self.nw.is_none()
    }
    fn is_empty_north(&self) -> bool {
        self.n.is_none() && self.ne.is_none() && self.nw.is_none()
    }
    fn is_empty_south(&self) -> bool {
        self.s.is_none() && self.se.is_none() && self.sw.is_none()
    }
    fn is_empty_east(&self) -> bool {
        self.e.is_none() && self.se.is_none() && self.ne.is_none()
    }
    fn is_empty_west(&self) -> bool {
        self.nw.is_none() && self.w.is_none() && self.sw.is_none()
    }
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

    fn survey(&self, loc: &Point) -> Survey {
        let n = Point(loc.0, loc.1 - 1);
        let ne = Point(loc.0 + 1, loc.1 - 1);
        let e = Point(loc.0 + 1, loc.1);
        let se = Point(loc.0 + 1, loc.1 + 1);
        let s = Point(loc.0, loc.1 + 1);
        let sw = Point(loc.0 - 1, loc.1 + 1);
        let w = Point(loc.0 - 1, loc.1);
        let nw = Point(loc.0 - 1, loc.1 - 1);

        Survey {
            n: self.grid.get(&n),
            ne: self.grid.get(&ne),
            e: self.grid.get(&e),
            se: self.grid.get(&se),
            s: self.grid.get(&s),
            sw: self.grid.get(&sw),
            w: self.grid.get(&w),
            nw: self.grid.get(&nw),
        }
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

struct Move {
    from: Point,
    to: Point,
}

impl Iterator for Grove {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let mut moves = vec![];
        // first half
        for (point, elf) in self.grid.iter() {
            let survey = self.survey(point);
            if !survey.is_empty() {
                if survey.is_empty_north() {
                    moves.push(Move {
                        from: *point,
                        to: Point(point.0, point.1 - 1),
                    });
                }
                if survey.is_empty_south() {
                    moves.push(Move {
                        from: *point,
                        to: Point(point.0, point.1 + 1),
                    });
                }
                if survey.is_empty_east() {
                    moves.push(Move {
                        from: *point,
                        to: Point(point.0 + 1, point.1),
                    });
                }
                if survey.is_empty_north() {
                    moves.push(Move {
                        from: *point,
                        to: Point(point.0 - 1, point.1),
                    });
                }
            }
        }

        // create a historgram of moves
        let mut histo = HashMap::new();
        for mov in &moves {
            let count = histo.entry(mov.to).or_insert(0);
            *count += 1;
        }

        // filter out any moves that appeared more than once
        histo = histo.drain_filter(|k, v| *v == 1).collect();

        // apply the remaining moves
        for mov in &moves {
            // if the move occurred only once
            if histo.contains_key(&mov.to) {
                // remove the elf at mov.from from self.grid and reinsert it at mov.to
                if let Some(elf) = self.grid.remove(&mov.from) {
                    self.grid.insert(mov.to, elf);
                }
            }
        }

        // second half

        Some(())
    }
}

impl Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PAD: i32 = 3;
        let bounds = self.bounding_box();
        for y in -PAD..=(bounds.height+PAD) {
            for x in -PAD..=(bounds.width+PAD) {
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
    grove.next();

    println!("{}", grove);
    grove.next();

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
