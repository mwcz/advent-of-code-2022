type Parsed = String;

pub fn parse(input: String) -> Parsed {
    input
}

pub fn part1(input: Parsed) -> usize {
    find_marker::<4>(&input)
}

pub fn part2(input: Parsed) -> usize {
    find_marker::<14>(&input)
}

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
