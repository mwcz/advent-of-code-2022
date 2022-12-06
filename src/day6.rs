use aoc_runner_derive::aoc;

fn find_marker<const MARKER_LEN: usize>(line: &str) -> usize {
    let chars: Vec<char> = line.chars().collect();

    let mut letters_seen: [bool; 26] = [false; 26];

    'win: for (i, win) in chars.windows(MARKER_LEN).enumerate() {
        // clear the letters seen
        letters_seen.iter_mut().for_each(|l| *l = false);

        for c in win {
            let ci = *c as usize - 97;
            if letters_seen[ci] {
                continue 'win;
            }
            letters_seen[ci] = true;
        }

        return i + MARKER_LEN;
    }

    unreachable!();
}

#[aoc(day6, part1)]
fn part1_solve(input: &str) -> usize {
    find_marker::<4>(input)
}

#[aoc(day6, part2)]
fn part2_solve(input: &str) -> usize {
    find_marker::<14>(input)
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn part1_parse_test() {
        assert_eq!(find_marker::<4>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_marker::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_marker::<4>("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_marker::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_marker::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
