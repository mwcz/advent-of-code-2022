use aoc_runner_derive::{aoc, aoc_generator};

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// The inherent score of this move.
    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    /// Play this move versus another move and return the score earned.
    fn versus(&self, other: &Self) -> u32 {
        self.score() + match (self, other) {
            (Move::Rock, Move::Rock) => 3,
            (Move::Rock, Move::Paper) => 0,
            (Move::Rock, Move::Scissors) => 6,
            (Move::Paper, Move::Rock) => 6 ,
            (Move::Paper, Move::Paper) => 3 ,
            (Move::Paper, Move::Scissors) => 0,
            (Move::Scissors, Move::Rock) => 0 ,
            (Move::Scissors, Move::Paper) => 6 ,
            (Move::Scissors, Move::Scissors) => 3,
        } 
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
}

#[aoc_generator(day2, part1)]
fn part1_gen(input: &str) -> Vec<(Move, Move)> {
    let mut guide: Vec<(Move, Move)> = vec![];
    for line in input.lines() {
        let mut pair = line.split_whitespace().map(|m| m.into());
        guide.push((pair.next().unwrap(), pair.next().unwrap()));
    }
    guide
}

#[aoc(day2, part1)]
fn part1(guide: &Vec<(Move, Move)>) -> u32 {
    let mut score = 0;
    for (theirs, yours) in guide {
        score += yours.versus(&theirs);
    }
    score
}

enum Outcome {
    Win,
    Lose,
    Draw
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
             "X" => Self::Lose,
             "Y" => Self::Draw,
             "Z" => Self::Win,
            _ => panic!(),
        }
    }
}

impl Outcome {
    fn choose(&self, their_move: &Move) -> Move {
        match (self, their_move) {
            (Outcome::Win, Move::Rock) => Move::Paper,
            (Outcome::Win, Move::Paper) => Move::Scissors,
            (Outcome::Win, Move::Scissors) => Move::Rock,
            (Outcome::Lose, Move::Rock) => Move::Scissors,
            (Outcome::Lose, Move::Paper) => Move::Rock,
            (Outcome::Lose, Move::Scissors) => Move::Paper,
            (Outcome::Draw, Move::Rock) => Move::Rock,
            (Outcome::Draw, Move::Paper) => Move::Paper,
            (Outcome::Draw, Move::Scissors) => Move::Scissors,
        }
    }
}

#[aoc_generator(day2, part2)]
fn part2_gen(input: &str) -> Vec<(Move, Outcome)> {
    let mut guide: Vec<(Move, Outcome)> = vec![];
    for line in input.lines() {
        let mut line_components = line.split_whitespace();
        guide.push((line_components.next().unwrap().into(), line_components.next().unwrap().into()));
    }
    guide
}


#[aoc(day2, part2)]
fn part2(guide: &Vec<(Move, Outcome)>) -> u32 {
    let mut score = 0;
    for (theirs, outcome) in guide {
        let yours = outcome.choose(theirs);
        score += yours.versus(&theirs);
    }
    score
}
