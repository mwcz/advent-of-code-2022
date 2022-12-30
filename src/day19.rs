use aoc_runner_derive::aoc;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Amount {
    ore: i32,
    clay: i32,
    obs: i32,
    geo: i32,
}

impl Amount {
    fn new(ore: i32, clay: i32, obs: i32, geo: i32) -> Self {
        Self {
            ore,
            clay,
            obs,
            geo,
        }
    }
}

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obs: self.obs + rhs.obs,
            geo: self.geo + rhs.geo,
        }
    }
}

impl Sub for Amount {
    type Output = Amount;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obs: self.obs - rhs.obs,
            geo: self.geo - rhs.geo,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Blueprint {
    id: i32,
    ore_bot: Amount,
    clay_bot: Amount,
    obs_bot: Amount,
    geo_bot: Amount,
}

#[derive(Debug, Eq, PartialEq)]
enum Bot {
    Ore,
    Clay,
    Obs,
    Geo,
}

impl Blueprint {
    fn new(id: usize, line: &str) -> Self {
        let mut words = line.split_whitespace();
        // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        // 0            1    2   3     4     5 6    7    8    9     10   11 12   13   14       15    16   17 18  19  20 21    22   23    24    25   26 27  28 29 30

        words.nth(5);

        let ore_bot_ore = words.next().unwrap().parse::<i32>().unwrap();

        words.nth(4);

        let clay_bot_ore = words.next().unwrap().parse::<i32>().unwrap();

        words.nth(4);

        let obs_bot_ore = words.next().unwrap().parse::<i32>().unwrap();

        words.nth(1);

        let obs_bot_clay = words.next().unwrap().parse::<i32>().unwrap();

        words.nth(4);

        let geo_bot_ore = words.next().unwrap().parse::<i32>().unwrap();

        words.nth(1);

        let geo_bot_obs = words.next().unwrap().parse::<i32>().unwrap();

        Self {
            id: id as i32,
            ore_bot: Amount::new(ore_bot_ore, 0, 0, 0),
            clay_bot: Amount::new(clay_bot_ore, 0, 0, 0),
            obs_bot: Amount::new(obs_bot_ore, obs_bot_clay, 0, 0),
            geo_bot: Amount::new(geo_bot_ore, 0, geo_bot_obs, 0),
        }
    }
}

#[aoc(day19, part1)]
fn part1_solve(input: &str) -> i32 {
    let blueprint: Vec<Blueprint> = input
        .lines()
        .enumerate()
        .map(|(i, line)| Blueprint::new(i + 1, line))
        .collect();

    // println!("{:#?}", blueprint);

    fn affordable(bp: &Blueprint, bot: &Bot, wallet: &Amount) -> (Amount, Amount) {
        let cost = match bot {
            // Bot::Wait => (Amount::zero(), Amount::zero()),
            Bot::Ore => (bp.ore_bot, Amount::new(1, 0, 0, 0)),
            Bot::Clay => (bp.clay_bot, Amount::new(0, 1, 0, 0)),
            Bot::Obs => (bp.obs_bot, Amount::new(0, 0, 1, 0)),
            Bot::Geo => (bp.geo_bot, Amount::new(0, 0, 0, 1)),
        };

        (
            Amount::new(
                wallet.ore - cost.0.ore,
                wallet.clay - cost.0.clay,
                wallet.obs - cost.0.obs,
                wallet.geo,
            ),
            cost.1,
        )
    }

    fn search<'a>(
        strats: &mut HashMap<&'a Blueprint, i32>,
        max: &mut i32,
        bp: &'a Blueprint,
        minute: i32,
        wallet: Amount,
        rate: Amount,
    ) {
        // base case
        if minute >= 24 {
            if wallet.geo > *max {
                println!(
                    "  blueprint {} got {} geodes at minute {}: quality score {}",
                    bp.id,
                    wallet.geo,
                    minute,
                    bp.id * wallet.geo
                );
                *max = wallet.geo;
                // strats.push((bp, wallet.geo));
                strats.insert(bp, wallet.geo);
            }
            return;
        }

        // // the most geos we could get if we build a geobot every remaining minute
        // let max_buildable = minute * (minute + 1) / 2;

        // // bail if we couldn't possibly make enough geo bots to match the current max
        // if wallet.geo + rate.geo * (minute - 1) + max_buildable <= *max {
        //     return;
        // }

        // spend

        // try each bot we can afford

        for (bot, spent_wallet, rate_adj) in [Bot::Geo, Bot::Obs, Bot::Clay, Bot::Ore]
            .iter()
            .filter_map(|bot| {
                // only include bots that can be afforded if we wait long enough at the current
                // rate (ex, if we aren't mining any obsidian, we can't afford a geobot no matter
                // how long we wait)
                if bot == &Bot::Obs && rate.clay == 0 {
                    None
                } else if bot == &Bot::Geo && rate.obs == 0 {
                    None
                } else {
                    let aff = affordable(bp, bot, &wallet);
                    Some((bot, aff.0, aff.1))
                }
            })
        {
            // time until the chosen bot is affordable
            let tt_aff = match bot {
                Bot::Ore => {
                    if wallet.ore > bp.ore_bot.ore {
                        1
                    } else {
                        (bp.ore_bot.ore - wallet.ore).div_ceil(rate.ore)
                    }
                }
                Bot::Clay => {
                    if wallet.ore > bp.clay_bot.ore {
                        1
                    } else {
                        (bp.clay_bot.ore - wallet.ore).div_ceil(rate.ore)
                    }
                }
                Bot::Obs => {
                    let tt_ore = if wallet.ore > bp.obs_bot.ore {
                        1
                    } else {
                        (bp.obs_bot.ore - wallet.ore).div_ceil(rate.ore)
                    };
                    let tt_clay = if wallet.clay > bp.obs_bot.clay {
                        1
                    } else {
                        (bp.obs_bot.clay - wallet.clay).div_ceil(rate.clay)
                    };

                    tt_ore.max(tt_clay)
                }
                Bot::Geo => {
                    let tt_ore = if wallet.ore > bp.geo_bot.ore {
                        1
                    } else {
                        (bp.geo_bot.ore - wallet.ore).div_ceil(rate.ore)
                    };
                    let tt_obs = if wallet.obs > bp.geo_bot.obs {
                        1
                    } else {
                        (bp.geo_bot.obs - wallet.obs).div_ceil(rate.obs)
                    };

                    tt_ore.max(tt_obs)
                }
            };

            // a time_remaining var that caps out at the maximum minutes
            let ttw = tt_aff.min(24 - minute);

            let new_rate = Amount::new(
                rate.ore + rate_adj.ore,
                rate.clay + rate_adj.clay,
                rate.obs + rate_adj.obs,
                rate.geo + rate_adj.geo,
            );

            let new_wallet = Amount::new(
                ttw * rate.ore + spent_wallet.ore,
                ttw * rate.clay + spent_wallet.clay,
                ttw * rate.obs + spent_wallet.obs,
                ttw * rate.geo + spent_wallet.geo,
            );

            if new_wallet.ore >= 0 && new_wallet.clay >= 0 && new_wallet.obs >= 0 {
                // keep searching if there's enough time left to implement this branch
                search(strats, max, bp, minute + ttw, new_wallet, new_rate);
            }
        }
    }

    let mut strats: HashMap<&Blueprint, i32> = HashMap::new();

    for bp in &blueprint {
        let wallet = Amount::new(0, 0, 0, 0);
        let rate = Amount::new(1, 0, 0, 0);
        let mut max = 0;

        println!("Analyzing blueprint {}", bp.id);

        search(&mut strats, &mut max, &bp, 0, wallet, rate);
    }

    strats.iter().map(|(bp, geos)| bp.id * geos).sum()
}

// #[aoc(day19, part2)]
// fn part2_solve(input: &str) -> usize {
//     todo!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solve(EX), 33);
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(part2_solve(EX), 58);
    // }
}
