use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
pub struct Supplies {
    stacks: Vec<Stack>,
    moves: Vec<Move>,
}

#[derive(Debug, PartialEq)]
struct Stack(Vec<char>);

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
        let from = spl.next().unwrap().parse::<usize>().unwrap();
        spl.next().unwrap(); // to
        let to = spl.next().unwrap().parse::<usize>().unwrap();
        Self { count, from, to }
    }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Supplies {
    let mut input_spl = input.split("\n\n");

    let stacks_input = input_spl.next().unwrap();
    let moves_input = input_spl.next().unwrap();

    Supplies {
        stacks: vec![],
        moves: moves_input.lines().map(Move::from).collect()
    }
}

#[aoc(day5, part1)]
fn part1_solve(pairs: &Supplies) -> usize {
    todo!();
}

#[aoc(day5, part2)]
fn part2_solve(pairs: &Supplies) -> usize {
    todo!();
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
            parse(SAMPLE_INPUT),
            Supplies {
                stacks: vec![],
                moves: vec![
                    Move { count: 1, from: 2, to: 1 },
                    Move { count: 3, from: 1, to: 3 },
                    Move { count: 2, from: 2, to: 1 },
                    Move { count: 1, from: 1, to: 2 },
                ]
            }
        );
    }

    // #[test]
    // fn part1_solve_test() {
    //     assert_eq!(
    //         part1_solve(&parse(SAMPLE_INPUT)),
    //         "CMZ"
    //     );
    // }

    // #[test]
    // fn part2_solve_test() {
    //     assert_eq!(
    //         part2_solve(&parse(
    //             "2-4,6-8\n\
    //             2-3,4-5\n\
    //             5-7,7-9\n\
    //             2-8,3-7\n\
    //             6-6,4-6\n\
    //             2-6,4-8"
    //         )),
    //         4
    //     );
    // }
}
