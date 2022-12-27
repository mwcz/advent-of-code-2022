use std::{
    collections::VecDeque,
    iter::{Cycle, Enumerate},
    slice::Iter,
    str::Chars,
};

use aoc_runner_derive::aoc;

const HASH_SIZE: usize = 60;

enum Shape {
    Plus,
    Corner,
    HLine,
    VLine,
    Square,
}

impl Shape {
    #[rustfmt::skip]
    fn mask(&self) -> [u8; 4] {
        match self {
            // these are upside down on purpose (so the bottom edge is at index 0)
            Shape::Plus => [
                0b0001000,
                0b0011100,
                0b0001000,
                0b0000000,
            ],
            Shape::Corner => [
                0b0011100,
                0b0000100,
                0b0000100,
                0b0000000,
            ],
            Shape::HLine => [
                0b0011110,
                0b0000000,
                0b0000000,
                0b0000000,
            ],
            Shape::VLine => [
                0b0010000,
                0b0010000,
                0b0010000,
                0b0010000,
            ],
            Shape::Square => [
                0b0011000,
                0b0011000,
                0b0000000,
                0b0000000,
            ],
        }
    }

    fn height(&self) -> usize {
        match self {
            Shape::Plus => 3,
            Shape::Corner => 3,
            Shape::HLine => 1,
            Shape::VLine => 4,
            Shape::Square => 2,
        }
    }
}

struct Chamber<'a> {
    /// The jets of hot gas.
    jets: Cycle<Enumerate<Chars<'a>>>,
    /// An infinite iterator that emits rock shapes in the prescribed order.
    shapes: Cycle<Enumerate<Iter<'a, Shape>>>,
    /// The settled rocks, represented by bits.
    rocks: VecDeque<u8>,
    /// The highest point in the chamber.
    peak: usize,
    /// How many rocks have fallen.
    rock_count: usize,
    /// How many rocks have been removed from the bottom.
    prune_count: usize,
}

impl<'a> Chamber<'a> {
    fn new(jets: &'a str) -> Self {
        let jets = jets.chars().enumerate().cycle();

        let shapes = [
            Shape::HLine,
            Shape::Plus,
            Shape::Corner,
            Shape::VLine,
            Shape::Square,
        ]
        .iter()
        .enumerate()
        .cycle();

        let rocks = VecDeque::with_capacity(HASH_SIZE);

        Self {
            shapes,
            jets,
            rocks,
            peak: 0,
            rock_count: 0,
            prune_count: 0,
        }
    }

    // fn to_string(&self, falling: Option<([u8; 4], usize)>) -> String {
    //     let mut out = "".to_string();
    //     out.push_str("\n      +-------+\n");
    //     for (y, rock) in self.rocks.iter().enumerate().rev() {
    //         out.push_str(&format!("{:5} ", y + self.prune_count));
    //         out.push_str(
    //             &format!("|{rock:07b}|\n")
    //                 .chars()
    //                 .enumerate()
    //                 .map(|(i, c)| {
    //                     if let Some((shape, shape_y)) = &falling {
    //                         for (row_y, row) in shape.iter().enumerate() {
    //                             let rowbits = format!("{row:07b}");
    //                             if (1..=7).contains(&i) {
    //                                 if shape_y + row_y == y
    //                                     && rowbits.chars().nth(i - 1).unwrap() == '1'
    //                                 {
    //                                     return '@';
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     if c == '1' {
    //                         '#'
    //                     } else if c == '0' {
    //                         '.'
    //                     } else {
    //                         c
    //                     }
    //                 })
    //                 .collect::<String>(),
    //         );
    //     }
    //     out.push_str("      +-------+");
    //     out
    // }

    fn row_collides(&self, y: usize, row: u8) -> bool {
        let hit_left_wall = row >= 0b10000000;
        // println!();
        // println!("{:#?}", self.rocks);
        // println!("{:#?}", self.rock_count);
        // println!("{:#?}", self.rocks.len());
        // println!("y: {:#?}", y);
        // println!("pc: {:#?}", self.prune_count);
        let row_at_rest = self.rocks[y - self.prune_count];
        let hit_rock = (row_at_rest & row) != 0;

        hit_left_wall || hit_rock
    }

    fn fill_space(&mut self, y: usize) {
        for _ in self.rocks.len()..(y - self.prune_count) {
            self.rocks.push_back(0);
        }
        // remove rocks in excess of the hash size
        if self.rocks.len() > HASH_SIZE {
            let prune_count = self.rocks.len() - HASH_SIZE;
            self.prune_count += prune_count;
            self.rocks.drain(0..prune_count);
        }
    }
}

impl Iterator for Chamber<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (_shape_idx, shape) = self.shapes.next().unwrap();
        let mut mask = shape.mask();

        // create a rock with the given shape
        // position it 4+ units _above_ the highest point
        //

        // the coordinates of the rock, anchored to the bottom-left corner of each rock mask.
        let mut y = self.peak + 3;

        // add new empty space above the peak
        self.fill_space(self.peak + 4 + 3);

        // println!("A new rock falls.");
        // println!("{}", self.to_string(Some((mask, y))));

        self.rock_count += 1;

        if self.rock_count % 10000000 == 0 {
            println!("{}", (self.rock_count as f32) / 1000000000000.0);
        }

        'outer: loop {
            // push
            let (_jet_idx, jet) = self.jets.next().unwrap();
            // println!("{}", self.to_string(Some((mask, y))));

            let mut shifted_mask = mask.map(|_| None);

            for (shape_y, row) in mask.iter().enumerate() {
                let new_row = if jet == '<' {
                    let would_hit_left_wall = row & 0b1000000 != 0;
                    if !would_hit_left_wall {
                        Some(row << 1)
                    } else {
                        None
                    }
                } else {
                    let would_hit_right_wall = row & 1 != 0;
                    if !would_hit_right_wall {
                        Some(row >> 1)
                    } else {
                        None
                    }
                };

                if let Some(new_row) = new_row {
                    if self.row_collides(shape_y + y, new_row) {
                        shifted_mask[shape_y] = None;
                    } else {
                        shifted_mask[shape_y] = Some(new_row);
                    }
                }
            }

            // print!("Jet of gas pushes rock ");
            // if jet == '<' {
            //     // print!("left");
            // } else {
            //     // print!("right");
            // }
            if shifted_mask.iter().all(|row| row.is_some()) {
                // println!(".");
                mask = shifted_mask.map(|row_opt| row_opt.unwrap());
                // println!("{}", self.to_string(Some((mask, y))));
            } else {
                // println!(", but nothing happens.");
            }
            // println!("{}", self.to_string(Some((mask, y))));

            // print!("Rock falls 1 unit");

            // fall
            //   decrement y and | with rocks in range
            //   if | > 0, rock is now resting
            if let Some(new_y) = y.checked_sub(1) {
                for (shape_y, row) in mask.iter().enumerate() {
                    if self.row_collides(shape_y + new_y, *row) {
                        // come to rest on another rock
                        // println!(", causing it to come to rest.");
                        break 'outer;
                    }
                }

                y = new_y;
            } else {
                // println!(", causing it to come to rest.");
                // come to rest at the floor
                break;
            }

            // println!(".");
        }

        // at rest
        //   update self.peak
        //   bitor mask into self.rocks
        for shape_y in 0..shape.height() {
            let new_row = mask[shape_y];
            self.rocks[y + shape_y - self.prune_count] |= new_row;
        }

        // store new peak height, if the new shape exceeds the current peak (shapes can come to
        // rest below the current peak)
        self.peak = self.peak.max(y + shape.height());

        // println!("{}", self.to_string(Some((mask, y))));

        Some(self.peak)
    }
}

#[aoc(day17, part1)]
fn part1_solve(input: &str) -> usize {
    let mut chamber = Chamber::new(input);

    chamber.nth(2022 - 1).unwrap()
}

#[aoc(day17, part2)]
fn part2_solve(input: &str) -> usize {
    let mut chamber = Chamber::new(input);

    chamber.nth(1000000000000 - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solve(EX), 3068);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solve(EX), 1514285714288);
    }
}
