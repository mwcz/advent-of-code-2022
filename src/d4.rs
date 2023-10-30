use nom::bytes::complete::tag;
use nom::character::complete::u32 as nomu32;
use nom::combinator::map;
use nom::{sequence::separated_pair, IResult};
use std::ops::RangeInclusive;

type Parsed = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;

pub fn parse(input: String) -> Parsed {
    type ParseRange = (u32, u32);
    type Line = (ParseRange, ParseRange);

    fn range(r: &str) -> IResult<&str, ParseRange> {
        separated_pair(nomu32, tag("-"), nomu32)(r)
    }

    fn line(line_str: &str) -> IResult<&str, Line> {
        separated_pair(range, tag(","), range)(line_str)
    }

    fn line_to_ranges(line_str: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
        map(line, |r| {
            (
                RangeInclusive::new(r.0 .0, r.0 .1),
                RangeInclusive::new(r.1 .0, r.1 .1),
            )
        })(line_str)
        .expect("")
        .1
    }

    input.lines().map(line_to_ranges).collect()
}

pub fn part1(pairs: Parsed) -> usize {
    pairs
        .iter()
        .filter(|pair| pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0))
        .count()
}

pub fn part2(pairs: Parsed) -> usize {
    pairs.iter().filter(|pair| pair.0.overlaps(&pair.1)).count()
}

trait RangeTools {
    fn fully_contains(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeTools for RangeInclusive<u32> {
    fn fully_contains(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.end() >= other.start() && self.start() <= other.end()
    }
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn part1_solve_test() {
        assert_eq!(
            part1(parse(
                "2-4,6-8\n\
                2-3,4-5\n\
                5-7,7-9\n\
                2-8,3-7\n\
                6-6,4-6\n\
                2-6,4-8"
                    .to_string()
            )),
            2
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(parse(
                "2-4,6-8\n\
                2-3,4-5\n\
                5-7,7-9\n\
                2-8,3-7\n\
                6-6,4-6\n\
                2-6,4-8"
                    .to_string()
            )),
            4
        );
    }
}
