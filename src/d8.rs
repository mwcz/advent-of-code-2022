type Parsed = [[i8; 99]; 99];

pub fn parse(input: String) -> Parsed {
    parse_with::<99>(input)
}

pub fn part1(trees: Parsed) -> usize {
    let mut forest = Forest::<99> {
        trees,
        visible: [[false; 99]; 99],
    };
    forest.count_visible()
}

pub fn part2(trees: Parsed) -> usize {
    let forest = Forest::<99> {
        trees,
        visible: [[false; 99]; 99],
    };
    forest.max_scenic()
}

fn parse_with<const SIZE: usize>(input: String) -> [[i8; SIZE]; SIZE] {
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

    fn max_scenic(&self) -> usize {
        let mut score = 0;
        for y in 1..(SIZE - 1) {
            for x in 1..(SIZE - 1) {
                score = score.max(self.scenic_score(x, y));
            }
        }
        score
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        // top right bottom left
        let mut scores = [0, 0, 0, 0];
        let height = self.trees[y][x];
        // left
        for left_idx in (0..x).rev() {
            scores[3] += 1;
            if self.trees[y][left_idx] >= height {
                break;
            }
        }
        for right_idx in (x + 1)..SIZE {
            scores[1] += 1;
            if self.trees[y][right_idx] >= height {
                break;
            }
        }
        for top_idx in (0..y).rev() {
            scores[0] += 1;
            if self.trees[top_idx][x] >= height {
                break;
            }
        }
        for bottom_idx in (y + 1)..SIZE {
            scores[2] += 1;
            if self.trees[bottom_idx][x] >= height {
                break;
            }
        }
        scores.iter().product()
    }
}

#[test]
fn day8_test() {
    assert_eq!(
        parse_with::<5>(
            "30373\n\
             25512\n\
             65332\n\
             33549\n\
             35390"
                .to_string(),
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
            trees: parse_with::<5>(
                "30373\n\
             25512\n\
             65332\n\
             33549\n\
             35390"
                    .to_string(),
            ),
            visible: [[false; 5]; 5],
        }
        .count_visible(),
        21
    );
    assert_eq!(
        Forest::<5> {
            trees: parse_with::<5>(
                "30373\n\
             25512\n\
             65332\n\
             33549\n\
             35390"
                    .to_string(),
            ),
            visible: [[false; 5]; 5],
        }
        .max_scenic(),
        8
    );
}
