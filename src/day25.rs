use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::{repeat_n, CombinationsWithReplacement, Itertools};
use rayon::prelude::*;

fn snafu(enc: &str) -> i64 {
    enc.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            5_i64.pow(i as u32)
                * match c {
                    '-' => -1,
                    '=' => -2,
                    c => c.to_string().parse().unwrap(),
                }
        })
        .sum()
}
fn ufans(num: i64) -> String {
    // num.to_string().chars().rev()

    let mut num = num;
    let mut chars: Vec<char> = vec![];
    let mut pow = 5_i64.pow(chars.len() as u32);

    // 976
    // 2=-01
    //
    //  2 * 5^4
    // -2 * 5^3
    // -1 * 5^2
    //  0 * 5^1
    //  1 * 5^0

    loop {
        if num < pow {
            pow /= 5;
        }

        if num == 0 {
            break;
        }
    }

    todo!();
}

fn part1_solve(input: &str) -> i64 {
    let chars = ['-', '=', '0', '1', '2'];

    // build a lookup table of all possible snafus

    let mut snafu_lut: HashMap<i64, String> = HashMap::new();
    let range = (1..=20).into_par_iter();
    let all_snafus: Vec<Vec<String>> = range.map(|i| {
        /*
            let snafus = chars
                .iter()
                .combinations_with_replacement(i)
        */
        let snafus: Vec<String> = repeat_n(chars.iter(), i)
            .multi_cartesian_product()
            .map(|chars| chars.into_iter().collect::<String>())
            .collect();
        println!("{i} done", );

        snafus
    }).collect();

    println!("{:#?}", snafu_lut.get(&976).unwrap());

    todo!();
}

#[aoc(day25, part1)]
fn part1_solver(input: &str) -> i64 {
    part1_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL: &str = include_str!("../input/2022/day25.txt");
    const EX: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn day25_part1_snafu() {
        assert_eq!(snafu("1"), 1);
        assert_eq!(snafu("2"), 2);
        assert_eq!(snafu("1="), 3);
        assert_eq!(snafu("1-"), 4);
        assert_eq!(snafu("10"), 5);
        assert_eq!(snafu("11"), 6);
        assert_eq!(snafu("12"), 7);
        assert_eq!(snafu("2="), 8);
        assert_eq!(snafu("2-"), 9);
        assert_eq!(snafu("20"), 10);
        assert_eq!(snafu("1=0"), 15);
        assert_eq!(snafu("1-0"), 20);
        assert_eq!(snafu("1=11-2"), 2022);
        assert_eq!(snafu("1-0---0"), 12345);
        assert_eq!(snafu("1121-1110-1=0"), 314159265);
        assert_eq!(snafu("1=-0-2"), 1747);
        assert_eq!(snafu("12111"), 906);
        assert_eq!(snafu("2=0="), 198);
        assert_eq!(snafu("21"), 11);
        assert_eq!(snafu("2=01"), 201);
        assert_eq!(snafu("111"), 31);
        assert_eq!(snafu("20012"), 1257);
        assert_eq!(snafu("112"), 32);
        assert_eq!(snafu("1=-1="), 353);
        assert_eq!(snafu("1-12"), 107);
        assert_eq!(snafu("12"), 7);
        assert_eq!(snafu("1="), 3);
        assert_eq!(snafu("122"), 37);
    }
    #[test]
    fn day25_part1_ufans() {
        assert_eq!(ufans(1), "1");
        assert_eq!(ufans(2), "2");
        assert_eq!(ufans(3), "1=");
        assert_eq!(ufans(4), "1-");
        assert_eq!(ufans(5), "10");
        assert_eq!(ufans(6), "11");
        assert_eq!(ufans(7), "12");
        assert_eq!(ufans(8), "2=");
        assert_eq!(ufans(9), "2-");
        assert_eq!(ufans(10), "20");
        assert_eq!(ufans(15), "1=0");
        assert_eq!(ufans(20), "1-0");
        assert_eq!(ufans(2022), "1=11-2");
        assert_eq!(ufans(12345), "1-0---0");
        assert_eq!(ufans(314159265), "1121-1110-1=0");
        assert_eq!(ufans(1747), "1=-0-2");
        assert_eq!(ufans(906), "12111");
        assert_eq!(ufans(198), "2=0=");
        assert_eq!(ufans(11), "21");
        assert_eq!(ufans(201), "2=01");
        assert_eq!(ufans(31), "111");
        assert_eq!(ufans(1257), "20012");
        assert_eq!(ufans(32), "112");
        assert_eq!(ufans(353), "1=-1=");
        assert_eq!(ufans(107), "1-12");
        assert_eq!(ufans(7), "12");
        assert_eq!(ufans(3), "1=");
        assert_eq!(ufans(37), "122");
    }
    #[test]
    fn day25_part1_example() {
        assert_eq!(part1_solve(EX), 4890);
    }
    #[test]
    fn day25_part1_real() {
        assert_eq!(part1_solve(REAL), 290);
    }
}
