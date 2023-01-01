use aoc_runner_derive::aoc;
use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
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
    fn in_the_black(&self) -> bool {
        // don't bother checking for geos, they'll always be >= 0 since we can't spend them
        self.ore >= 0 && self.clay >= 0 && self.obs >= 0
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

impl Mul<i32> for Amount {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obs: self.obs * rhs,
            geo: self.geo * rhs,
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Bot {
    Ore,
    Clay,
    Obs,
    Geo,
}

impl Bot {
    fn cost<'a>(&'a self, bp: &'a Blueprint) -> Amount {
        match self {
            Bot::Ore => bp.ore_bot,
            Bot::Clay => bp.clay_bot,
            Bot::Obs => bp.obs_bot,
            Bot::Geo => bp.geo_bot,
        }
    }
    fn rate<'a>(&'a self) -> Amount {
        match self {
            Bot::Ore => Amount::new(1, 0, 0, 0),
            Bot::Clay => Amount::new(0, 1, 0, 0),
            Bot::Obs => Amount::new(0, 0, 1, 0),
            Bot::Geo => Amount::new(0, 0, 0, 1),
        }
    }
    fn time_to_build(&self, wallet: &Amount, rate: &Amount, bp: &Blueprint) -> i32 {
        match self {
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
        }
    }
}

impl Blueprint {
    fn new(id: usize, line: &str) -> Self {
        let mut words = line.split_whitespace();

        // Example line:
        // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.

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

    fn search<'a>(
        strats: &mut HashMap<&'a Blueprint, i32>,
        steps: Vec<(Bot, i32)>,
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
                println!("    Steps: {:?}", steps);
                *max = wallet.geo;
                // strats.push((bp, wallet.geo));
                strats.insert(bp, wallet.geo);
            }
            return;
        }

        for (bot, spent_wallet) in [Bot::Geo, Bot::Obs, Bot::Clay, Bot::Ore]
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
                    // buy the bot (can result in negative wallet values; check for that later
                    Some((bot, wallet - bot.cost(bp)))
                }
            })
        {
            // time until the chosen bot is affordable
            let time_to_build = bot.time_to_build(&wallet, &rate, bp);

            if minute + time_to_build > 24 {
                // not enough time to buy this bot, so just spin down the clock at the current rate
                // (the call to search will catch the base case)
                let new_wallet = wallet + rate * (24 - minute);
                search(strats, steps.clone(), max, bp, 24, new_wallet, rate);
            } else {
                let new_wallet = spent_wallet + rate * time_to_build;
                let new_rate = rate + bot.rate();
                let new_minute = minute + time_to_build;

                let mut new_steps = steps.clone();
                new_steps.push( (*bot, new_minute + 1) );
                // if new_wallet.in_the_black() {
                    search(strats, new_steps, max, bp, new_minute, new_wallet, new_rate);
                // }
            }
        }
    }

    let mut strats: HashMap<&Blueprint, i32> = HashMap::new();

    for bp in &blueprint {
        let wallet = Amount::new(0, 0, 0, 0);
        let rate = Amount::new(1, 0, 0, 0);
        let mut max = 0;

        println!("Analyzing {:#?}\n", bp);

        search(&mut strats, vec![], &mut max, bp, 0, wallet, rate);
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
