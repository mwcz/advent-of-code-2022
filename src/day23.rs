use aoc_runner_derive::aoc;
use console_engine::{ConsoleEngine, KeyCode};
use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::{array::IntoIter, collections::HashMap, fmt::Display, iter::Cycle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Sub, SubAssign)]
struct Point(i32, i32);

impl Point {
    fn north() -> Self {
        Point(0, -1)
    }
    fn south() -> Self {
        Point(0, 1)
    }
    fn east() -> Self {
        Point(1, 0)
    }
    fn west() -> Self {
        Point(-1, 0)
    }
}

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
    /// Is the cardinal direction empty?  ie, given north, is the space nw, n, and ne empty.
    fn is_empty_in_dir(&self, dir: char) -> bool {
        match dir {
            'n' => self.is_empty_north(),
            's' => self.is_empty_south(),
            'e' => self.is_empty_east(),
            'w' => self.is_empty_west(),
            _ => panic!("invalid dir '{dir}' passed to is_empty_in_dir"),
        }
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
    directions: Cycle<IntoIter<(char, Point), 4>>,
}

impl Grove {
    fn new(grid: HashMap<Point, Elf>) -> Self {
        Self {
            grid,
            directions: [
                ('n', Point::north()),
                ('s', Point::south()),
                ('w', Point::west()),
                ('e', Point::east()),
            ]
            .into_iter()
            .cycle(),
        }
    }

    fn survey(&self, loc: Point) -> Survey {
        let n = loc + Point::north();
        let ne = loc + Point::north() + Point::east();
        let e = loc + Point::east();
        let se = loc + Point::south() + Point::east();
        let s = loc + Point::south();
        let sw = loc + Point::south() + Point::west();
        let w = loc + Point::west();
        let nw = loc + Point::north() + Point::west();

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
            width: maxx - minx + 1,
            height: maxy - miny + 1,
            origin: Point(minx, miny),
        }
    }
}

struct Move {
    from: Point,
    to: Point,
}

impl Iterator for Grove {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let mut elf_moved = false;
        let mut moves = vec![];
        // first half
        for (point, _) in self.grid.iter() {
            let mut dirs = self.directions.clone();
            let survey = self.survey(*point);
            if !survey.is_empty() {
                for _ in 0..4 {
                    let (dirname, dir) = dirs.next().unwrap();
                    if survey.is_empty_in_dir(dirname) {
                        moves.push(Move {
                            from: *point,
                            to: *point + dir,
                        });
                        break; // done with this elf
                    }
                }
            }
        }

        // create a historgram of moves
        let mut histo = HashMap::new();
        for mov in &moves {
            let count = histo.entry(mov.to).or_insert(0);
            *count += 1;
        }

        // second half
        // apply the remaining moves
        for mov in &moves {
            // if the move occurred only once
            let mov_count = histo.get(&mov.to);
            if let Some(1) = mov_count {
                // remove the elf at mov.from from self.grid and reinsert it at mov.to
                if let Some(elf) = self.grid.remove(&mov.from) {
                    self.grid.insert(mov.to, elf);
                    elf_moved = true;
                }
            }
        }

        // cycle away the first direction considered this round
        self.directions.next();

        Some(elf_moved)
    }
}

impl Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PAD: i32 = 3;
        let bounds = self.bounding_box();
        for y in (bounds.origin.1 - PAD)..=(bounds.origin.1 + bounds.height + PAD) {
            for x in (bounds.origin.0 - PAD)..=(bounds.origin.0 + bounds.width + PAD) {
                let cell = self.grid.get(&Point(x, y));
                let symbol = match cell {
                    Some(_) => '#',
                    None => '.',
                };
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Grove {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((Point(x as i32, y as i32), Elf { proposal: None }))
                } else {
                    None
                }
            })
        })
        .collect::<HashMap<Point, Elf>>();

    Grove::new(grid)
}

fn part1_solve(input: &str, rounds: usize) -> i32 {
    let mut grove = parse(input);

    grove.nth(rounds - 1); // offset 1 to account for initial state

    // let area = grove.grid.len() * grove.grid[0].len();
    let bounds = grove.bounding_box();
    let area = bounds.width * bounds.height;
    let elf_count = grove.grid.len() as i32;

    area - elf_count
}

#[aoc(day23, part1)]
fn part1_solver(input: &str) -> i32 {
    part1_solve(input, 10)
}

fn part2_solve(input: &str) -> i32 {
    let mut grove = parse(input);

    // let (rounds, _) = grove.into_iter().enumerate().find(|(_, p)| !p).unwrap();
    // rounds + 1

    let width = 140;
    let height = 140;
    let fps = 250;
    #[cfg(feature = "visualize")]
    let mut engine = ConsoleEngine::init(width, height, fps).unwrap();

    #[cfg(feature = "visualize")]
    let print_grid = |round: i32, grove: &Grove, engine: &mut ConsoleEngine| {
        engine.wait_frame();
        engine.clear_screen();

        engine.print(0, 0, &format!("Round {}", round));
        engine.print(0, 2, &format!("{grove}"));

        engine.draw();
    };

    let mut rounds = 1;
    while let Some(true) = grove.next() {
        #[cfg(feature = "visualize")]
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        #[cfg(feature = "visualize")]
        print_grid(rounds, &grove, &mut engine);
        rounds += 1;
    }

    println!("{grove}",);

    rounds
}

#[aoc(day23, part2)]
fn part2_solver(input: &str) -> i32 {
    part2_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL: &str = include_str!("../input/2022/day23.txt");
    // const EX: &str = ".....\n\
    //                   ..##.\n\
    //                   ..#..\n\
    //                   .....\n\
    //                   ..##.\n\
    //                   .....";

    const EX2: &str = "....#..\n\
                       ..###.#\n\
                       #...#.#\n\
                       .#...##\n\
                       #.###..\n\
                       ##.#.##\n\
                       .#..#..";

    #[test]
    fn day23_part1_example() {
        assert_eq!(part1_solve(EX2, 10), 110);
    }

    #[test]
    fn day23_part2_example() {
        assert_eq!(part2_solve(EX2), 20);
    }

    #[test]
    fn day23_part2_real() {
        assert_eq!(part2_solve(REAL), 988);
    }
}
