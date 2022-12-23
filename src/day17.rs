use std::{iter::Cycle, slice::Iter};

use aoc_runner_derive::aoc;

struct Rock {
    shape: Shape,
    pos: Point,
}

struct Point {
    x: u32,
    y: u32,
}

enum Shape {
    Plus,
    Corner,
    HLine,
    VLine,
    Square,
}

impl Shape {
    fn mask(&self) -> [u8; 4] {
        match self {
            Shape::Plus => {
                [
                    0b00000000,
                    0b01000000,
                    0b11100000,
                    0b01000000,
                ]
            }
            Shape::Corner => {
                [
                    0b00000000,
                    0b00100000,
                    0b00100000,
                    0b11100000,
                ]
            }
            Shape::HLine => {
                [
                    0b00000000,
                    0b00000000,
                    0b00000000,
                    0b11110000,
                ]
            }
            Shape::VLine => {
                [
                    0b10000000,
                    0b10000000,
                    0b10000000,
                    0b10000000,
                ]
            }
            Shape::Square => {
                [
                    0b00000000,
                    0b00000000,
                    0b11000000,
                    0b11000000,
                ]
            }
        }
    }
}

struct Chamber<'a> {
    /// The jets of hot gas.
    jets: &'a str,
    /// An infinite iterator that emits rock shapes in the prescribed order.
    shapes: Cycle<Iter<'a, Shape>>,
    /// The settled rocks, represented by bits.
    rocks: Vec<u8>,
}

impl<'a> Chamber<'a> {
    fn new(jets: &'a str) -> Self {
        let shapes = [
            Shape::HLine,
            Shape::Plus,
            Shape::Corner,
            Shape::VLine,
            Shape::Square,
        ]
        .iter()
        .cycle();

        let rocks = vec![
            0b00000010,
            0b11000010,
            0b11001110,
        ];

        Self {
            shapes,
            jets,
            rocks,
        }
    }

    fn to_string(&self) -> String {
        let mut out = "Rock Chamber\n".to_string();
        for rock in &self.rocks {
            out.push_str(&format!("{rock:010b}").chars().map(|c| if c == '1' { 'O' } else { '_' }).collect::<String>());
            out.push_str("\n");
        }
        out
    }
}

impl Iterator for Chamber<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let shape = self.shapes.next().unwrap();

        // create a rock with the given shape
        // position it 4+ units _above_ the highest point
        //

        // the coordinates of the rock, anchored to the bottom-left corner of each rock mask.
        let mut x = 2; // 
        let mut y = 0;

        todo!();
    }
}

#[aoc(day17, part1)]
fn part1_solve(input: &str) -> u32 {
    let mut chamber = Chamber::new(input);

    println!("{}", chamber.to_string());

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solve(EX), 3068);
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(part2_solve(EX, 20), 56000011);
    // }
}
