type Parsed = String;

pub fn parse(input: String) -> Parsed {
    input
}

fn part1_parse(input: String) -> Vec<(Move, Move)> {
    let mut guide: Vec<(Move, Move)> = vec![];
    for line in input.lines() {
        let mut pair = line.split_whitespace().map(|m| m.into());
        guide.push((pair.next().unwrap(), pair.next().unwrap()));
    }
    guide
}

fn part2_parse(input: String) -> Vec<(Move, Outcome)> {
    let mut guide: Vec<(Move, Outcome)> = vec![];
    for line in input.lines() {
        let mut line_components = line.split_whitespace();
        guide.push((
            line_components.next().unwrap().into(),
            line_components.next().unwrap().into(),
        ));
    }
    guide
}

pub fn part1(input: Parsed) -> u32 {
    let guide = part1_parse(input);
    let mut score = 0;
    for (theirs, yours) in guide.iter() {
        score += yours.versus(theirs);
    }
    score
}

pub fn part2(input: Parsed) -> u32 {
    let guide = part2_parse(input);
    let mut score = 0;
    for (theirs, outcome) in guide.iter() {
        let yours = outcome.choose(theirs);
        score += yours.versus(theirs);
    }
    score
}

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
        self.score()
            + match (self, other) {
                (Move::Rock, Move::Rock) => 3,
                (Move::Rock, Move::Paper) => 0,
                (Move::Rock, Move::Scissors) => 6,
                (Move::Paper, Move::Rock) => 6,
                (Move::Paper, Move::Paper) => 3,
                (Move::Paper, Move::Scissors) => 0,
                (Move::Scissors, Move::Rock) => 0,
                (Move::Scissors, Move::Paper) => 6,
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

enum Outcome {
    Win,
    Lose,
    Draw,
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
