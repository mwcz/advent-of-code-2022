use aoc_runner_derive::{aoc, aoc_generator};

type Packs = [u32; 3];

#[aoc_generator(day1)]
pub fn part1_gen(input: &str) -> Packs {
    let mut packs: [u32; 3] = [0, 0, 0];
    let mut pack: u32 = 0;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(line_cals) => pack += line_cals,
            Err(_) => {
                // blank line, start a new pack
                for seen_pack in packs.iter_mut() {
                    if pack > *seen_pack {
                        *seen_pack = pack;
                        break;
                    }
                }
                pack = 0;
                packs.sort();
            }
        }
    }

    packs.sort();

    packs
}

#[aoc(day1, part1)]
pub fn part1(packs: &Packs) -> u32 {
    *packs.last().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(packs: &Packs) -> u32 {
    packs.iter().sum()
}
