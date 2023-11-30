type Parsed = String;

pub fn parse(input: String) -> Parsed {
    input
}

pub fn part1(input: Parsed) -> String {
    ufans(input.lines().map(snafu).sum())
}

pub fn part2(_input: Parsed) -> &'static str {
    "merry christmas!"
}

fn snafu(enc: &str) -> i128 {
    enc.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            5_i128.pow(i as u32)
                * match c {
                    '-' => -1,
                    '=' => -2,
                    c => c.to_string().parse().unwrap(),
                }
        })
        .sum()
}
fn ufans(num: i128) -> String {
    let mut chars: Vec<char> = vec![];
    let max_exp = num.to_string().len() as u32; // maximum exponent of 5 to consider for this number
    let mut num = num;

    while num != 0 {
        for exp in (0..=max_exp * 4).rev() {
            let coefs = [-2, -1, 0, 1, 2];
            let values = coefs.map(|coef| {
                let pow = coef * 5_i128.wrapping_pow(exp);
                let dist = (num - pow).abs();
                (coef, pow, dist, exp)
            });
            let best = values.iter().min_by(|a, b| a.2.cmp(&b.2));
            let best_coef = best.unwrap().0;

            // don't add leading zeroes
            if !(chars.is_empty() && best_coef == 0) {
                chars.push(match best_coef {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => panic!("invalid coefficient"),
                });
                // println!("{best:?}");
                num -= best.unwrap().1;
            }
        }
    }

    chars.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL: &str = include_str!("../input/d25");
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
    fn day25_snafu() {
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
    fn day25_ufans() {
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
        assert_eq!(part1(EX.to_string()), "2=-1=0".to_string());
    }
    #[test]
    fn day25_part1_real() {
        assert_eq!(part1(REAL.to_string()), "0".to_string());
    }
}
