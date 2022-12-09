use aoc_runner_derive::aoc;

fn parse<const SIZE: usize>(input: &str) -> [[i8; SIZE]; SIZE] {
    let mut trees = [[0; SIZE]; SIZE];
    const ASCII_DEC_START: i8 = 48;

    for (line_idx, line) in input.lines().enumerate() {
        for (height_idx, height) in line.chars().enumerate() {
            trees[line_idx][height_idx] = height as i8 - ASCII_DEC_START;
        }
    }

    trees
}

struct Forest<const SIZE: usize> {
    trees: [[i8; SIZE]; SIZE],
    visible: [[bool; SIZE]; SIZE],
}

impl<const SIZE: usize> Forest<SIZE> {
    fn count_visible(&mut self) -> usize {
        // TODO factor these loops more elegantly

        // col top to bottom
        for a in 1..(SIZE - 1) {
            let mut tallest = self.trees[0][a];
            // col TTB
            for b in 1..(SIZE - 1) {
                let tree = &self.trees[b][a];
                if *tree > tallest {
                    self.visible[b][a] = true;
                    if *tree == 9 {
                        break;
                    } else {
                        tallest = *tree;
                    }
                }
            }

            // col BTT
            tallest = self.trees[SIZE - 1][a];
            for b in (1..(SIZE - 1)).rev() {
                let tree = &self.trees[b][a];
                if *tree > tallest {
                    self.visible[b][a] = true;
                    if *tree == 9 {
                        break;
                    } else {
                        tallest = *tree;
                    }
                }
            }

            // row LTR
            tallest = self.trees[a][0];
            for b in 1..(SIZE - 1) {
                let tree = &self.trees[a][b];
                if *tree > tallest {
                    self.visible[a][b] = true;
                    if *tree == 9 {
                        break;
                    } else {
                        tallest = *tree;
                    }
                }
            }

            // col BTT
            tallest = self.trees[a][SIZE - 1];
            for b in (1..(SIZE - 1)).rev() {
                let tree = &self.trees[a][b];
                if *tree > tallest {
                    self.visible[a][b] = true;
                    if *tree == 9 {
                        break;
                    } else {
                        tallest = *tree;
                    }
                }
            }
        }

        self.visible
            .iter()
            .map(|row| row.iter().filter(|&t| *t).count())
            .sum::<usize>()
            + (SIZE - 1) * 4
    }

    fn get(&self, row: usize, col: usize) -> i8 {
        let Some(row) = self.trees.get(row) else {
            return -1;
        };

        let Some(tree) = row.get(col) else {
            return -1;
        };

        *tree
    }
}

#[aoc(day8, part1)]
fn part1_solve(input: &str) -> usize {
    let mut forest = Forest::<99> {
        trees: parse::<99>(input),
        visible: [[false; 99]; 99],
    };
    forest.count_visible()
}

#[aoc(day8, part2)]
fn part2_solve(input: &str) -> u32 {
    todo!();
}

#[test]
fn day8_test() {
    assert_eq!(
        parse::<5>(
            "30373\n\
             25512\n\
             65332\n\
             33549\n\
             35390",
        ),
        [
            [3, 0, 3, 7, 3,],
            [2, 5, 5, 1, 2,],
            [6, 5, 3, 3, 2,],
            [3, 3, 5, 4, 9,],
            [3, 5, 3, 9, 0,],
        ]
    );
    assert_eq!(
        Forest::<5> {
            trees: parse::<5>(
                "30373\n\
             25512\n\
             65332\n\
             33549\n\
             35390",
            ),
            visible: [[false; 5]; 5],
        }
        .count_visible(),
        21
    );
}
