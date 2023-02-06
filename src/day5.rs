use std::time::Duration;

use aoc_runner_derive::aoc;

#[cfg(feature = "visualize")]
use console_engine::{ConsoleEngine, KeyCode};

#[derive(Debug, PartialEq)]
pub struct Supplies<const STACK_COUNT: usize> {
    stacks: Vec<Stack>,
    moves: Vec<Move>,
}

type Stack = Vec<char>;

#[derive(Debug, PartialEq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let mut spl = value.split_whitespace();
        spl.next().unwrap(); // move
        let count = spl.next().unwrap().parse::<usize>().unwrap();
        spl.next().unwrap(); // from
        let from = spl.next().unwrap().parse::<usize>().unwrap() - 1;
        spl.next().unwrap(); // to
        let to = spl.next().unwrap().parse::<usize>().unwrap() - 1;
        Self { count, from, to }
    }
}

impl<const STACK_COUNT: usize> Supplies<STACK_COUNT> {
    fn parse(input: &str) -> Supplies<STACK_COUNT> {
        let mut input_spl = input.split("\n\n");

        let stacks_input = input_spl.next().unwrap();
        let moves_input = input_spl.next().unwrap();

        let stack_lines = stacks_input.lines();

        let mut stacks: Vec<Vec<char>> = vec![vec![]; STACK_COUNT];

        stack_lines.for_each(|line| {
            let mut chars = line.chars();

            chars.next(); // consume initial [

            // if the first char of the line is 1, then we've reached the labels.  otherwise,
            // document the stack items.
            for (i, c) in chars.step_by(4).enumerate() {
                if let 'A'..='Z' = c {
                    // add the char to the beginning of the vec, since they're listed in reverse order
                    stacks[i].insert(0, c);
                }
            }
        });

        Supplies {
            stacks,
            moves: moves_input.lines().map(Move::from).collect(),
        }
    }

    fn rearrange_9000(&mut self) {
        for mov in &self.moves {
            for _ in 0..mov.count {
                let from_crate = self.stacks[mov.from]
                    .pop()
                    .expect("can't move crate that doesn't exist");
                self.stacks[mov.to].push(from_crate);
            }
        }
    }

    fn rearrange_9001(&mut self) {
        #[cfg(feature = "visualize")]
        let print_grid = |stacks: &Vec<Vec<char>>, engine: &mut ConsoleEngine| {
            engine.wait_frame();
            engine.clear_screen();

            let mut output: Vec<String> = Vec::new();

            // find the tallest stack height H
            // loop over i from H to 0 and print the char from each stack that has a char at that i
            // label the stacks at the bottom

            let height = stacks.iter().map(|s| s.len()).max().unwrap();

            for h in (0..height).rev() {
                let mut line: Vec<String> = Vec::new();
                for s in stacks {
                    line.push(match s.get(h) {
                        Some(c) => format!("[{}] ", c),
                        None => "    ".into(),
                    });
                }
                output.push(line.join(""));
            }

            // add number labels to the stacks
            output.push((1..=stacks.len()).map(|n| format!(" {}  ", n)).collect());

            engine.print(0, (engine.get_height() - (height as u32)) as i32 - 2, &output.join("\n"));
            engine.draw();
        };

        #[cfg(feature = "visualize")]
        let fps = 4;
        #[cfg(feature = "visualize")]
        let term_height = 48; // this is enough to hold the characters up until the maximum height
                              // any of the stacks receives given my input.
        #[cfg(feature = "visualize")]
        let mut engine =
            ConsoleEngine::init((self.stacks.len() as u32) * 4, term_height, fps).unwrap();

        for mov in &self.moves {
            let from_len = self.stacks[mov.from].len();

            // reverse the elements about to be moved
            self.stacks[mov.from][from_len - mov.count..].reverse();

            for _ in 0..mov.count {
                let from_crate = self.stacks[mov.from]
                    .pop()
                    .expect("can't move crate that doesn't exist");
                self.stacks[mov.to].push(from_crate);

                #[cfg(feature = "visualize")]
                if engine.is_key_pressed(KeyCode::Char('q')) {
                    break;
                }
                #[cfg(feature = "visualize")]
                print_grid(&self.stacks, &mut engine);
            }
        }

        // keep the final on-screen for a bit before exiting
        #[cfg(feature = "visualize")]
        std::thread::sleep(Duration::from_millis(2000));
    }

    fn top_crates(&self) -> [char; STACK_COUNT] {
        let mut top_crates = [' '; STACK_COUNT];

        for (i, stack) in self.stacks.iter().enumerate() {
            top_crates[i] = *stack.last().expect("CRATERED!");
        }

        top_crates
    }
}

fn part1_solve<const STACK_COUNT: usize>(input: &str) -> String {
    let mut supplies = Supplies::<STACK_COUNT>::parse(input);

    supplies.rearrange_9000();

    supplies.top_crates().iter().cloned().collect()
}
#[aoc(day5, part1)]
fn part1_solver(input: &str) -> String {
    part1_solve::<9>(input)
}

fn part2_solve<const STACK_COUNT: usize>(input: &str) -> String {
    let mut supplies = Supplies::<STACK_COUNT>::parse(input);

    supplies.rearrange_9001();

    supplies.top_crates().iter().cloned().collect()
}
#[aoc(day5, part2)]
fn part2_solver(input: &str) -> String {
    part2_solve::<9>(input)
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    const REAL: &str = include_str!("../input/2022/day5.txt");

    const EX: &str = "    [D]    \n\
                      [N] [C]    \n\
                      [Z] [M] [P]\n\
                       1   2   3 \n\
                      \n\
                      move 1 from 2 to 1\n\
                      move 3 from 1 to 3\n\
                      move 2 from 2 to 1\n\
                      move 1 from 1 to 2";

    #[test]
    fn day5_part1_ex() {
        assert_eq!(part1_solve::<3>(EX), "CMZ");
    }
    #[test]
    fn day5_part1_real() {
        assert_eq!(part1_solve::<9>(REAL), "LBLVVTVLP");
    }
    #[test]
    fn day5_part2_ex() {
        assert_eq!(part2_solve::<3>(EX), "MCD");
    }
    #[test]
    fn day5_part2_real() {
        assert_eq!(part2_solve::<9>(REAL), "TPFFBDRJD");
    }
}
