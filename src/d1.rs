type Packs = [u32; 3];
type Parsed = Packs;

pub fn parse(input: String) -> Parsed {
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

    packs
}

pub fn part1(packs: Parsed) -> u32 {
    *packs.last().unwrap()
}

pub fn part2(packs: Parsed) -> u32 {
    packs.iter().sum()
}
