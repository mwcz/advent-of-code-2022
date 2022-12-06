use aoc_runner_derive::aoc;

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
        for mov in &self.moves {
            let from_len = self.stacks[mov.from].len();

            // reverse the elements about to be moved
            self.stacks[mov.from][from_len - mov.count..].reverse();

            for _ in 0..mov.count {
                let from_crate = self.stacks[mov.from]
                    .pop()
                    .expect("can't move crate that doesn't exist");
                self.stacks[mov.to].push(from_crate);
            }
        }
    }

    fn top_crates(&self) -> [char; STACK_COUNT] {
        let mut top_crates = [' '; STACK_COUNT];

        for (i, stack) in self.stacks.iter().enumerate() {
            top_crates[i] = *stack.last().expect("CRATERED!");
        }

        top_crates
    }
}

#[aoc(day5, part1)]
fn part1_solve(input: &str) -> String {
    let mut supplies = Supplies::<9>::parse(input);

    supplies.rearrange_9000();

    supplies.top_crates().iter().cloned().collect()
}

#[aoc(day5, part2)]
fn part2_solve(input: &str) -> String {
    let mut supplies = Supplies::<9>::parse(input);

    supplies.rearrange_9001();

    supplies.top_crates().iter().cloned().collect()
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    const SAMPLE_INPUT: &str = "    [D]    \n\
                                [N] [C]    \n\
                                [Z] [M] [P]\n\
                                 1   2   3 \n\
                                \n\
                                move 1 from 2 to 1\n\
                                move 3 from 1 to 3\n\
                                move 2 from 2 to 1\n\
                                move 1 from 1 to 2";

    #[test]
    fn part1_parse_test() {
        #[rustfmt::skip]
        assert_eq!(
            Supplies::<3>::parse(SAMPLE_INPUT),
            Supplies {
                stacks: vec![
                    vec!['Z', 'N'],
                    vec!['M', 'C', 'D'],
                    vec!['P'],
                ],
                moves: vec![
                    Move { count: 1, from: 1, to: 0 },
                    Move { count: 3, from: 0, to: 2 },
                    Move { count: 2, from: 1, to: 0 },
                    Move { count: 1, from: 0, to: 1 },
                ]
            }
        );
    }
}
