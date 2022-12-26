use aoc_runner_derive::aoc;
use itertools::Itertools;
use petgraph::algo::floyd_warshall;
use petgraph::prelude::*;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::collections::HashMap;

const START: &str = "AA";

#[derive(Debug)]
struct Valve<'name> {
    data: ValveData<'name>,
    exits: Vec<String>,
}

/// The data about a valve that's added to the graph as a node "weight".
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
struct ValveData<'name> {
    name: &'name str,
    rate: u32,
    mask: u32,
}

impl<'name> std::fmt::Debug for ValveData<'name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.name, self.rate))
    }
}

#[derive(Debug)]
struct Cave<'name> {
    graph: GraphMap<ValveData<'name>, u32, Directed>,
    start: Option<ValveData<'name>>,
    dists: Option<HashMap<(ValveData<'name>, ValveData<'name>), u32>>,
}

#[derive(Debug, Copy, Clone)]
struct Player<'name> {
    /// The valve the player is currently standing by.
    dest: Option<ValveData<'name>>,
    position: ValveData<'name>,
    /// Time left until this player can move again.
    ttm: u32,
}

impl<'name> Player<'name> {
    fn new(position: ValveData<'name>) -> Self {
        Self {
            position,
            dest: None,
            ttm: 0,
        }
    }
}

impl<'input> Cave<'input> {
    fn new(valves: Vec<Valve<'input>>) -> Self {
        let mut cave = Self {
            graph: GraphMap::new(),
            start: None,
            dists: None,
        };

        // add valve nodes

        for valve in &valves {
            if valve.data.name == START {
                cave.start = Some(valve.data);
            }
            if valve.data.name != START && valve.data.rate == 0 {
                continue;
            }
            let exits = valve
                .exits
                .iter()
                .filter_map(|e| valves.iter().find(|v| e == v.data.name));
            for exit in exits {
                cave.add_tunnel(&valve.data, &exit, &valves, &mut vec![valve.data.name]);
            }
        }

        // pre-compute distances between each pair of valves

        if let Ok(shortest_paths) = floyd_warshall(&cave.graph, |e| *e.weight()) {
            cave.dists = Some(
                shortest_paths
                    .iter()
                    .map(|path| {
                        if path.1 == &u32::MAX {
                            if let Some(opposite) = shortest_paths.get(&(path.0 .1, path.0 .0)) {
                                return ((path.0 .0, path.0 .1), *opposite);
                            }
                        }
                        return ((path.0 .0, path.0 .1), *path.1);
                    })
                    .collect(),
            );

            // remove all edges and replace them with edges connecting every node with every other
            // node, to eliminate the need to ever do pathfinding.

            for ((a, b), dist) in shortest_paths.iter() {
                cave.graph.remove_edge(*a, *b);
                if a != b {
                    cave.graph.add_edge(*a, *b, *dist);
                }
            }
        }

        cave
    }

    // add nodes and edges, but collapse nodes representing valves with 0 flow rate into edges.
    fn add_tunnel(
        &mut self,
        from: &ValveData<'input>,
        to: &Valve<'input>,
        valves: &Vec<Valve<'input>>,
        visited: &mut Vec<&'input str>,
    ) {
        self.graph.add_node(*from);

        // collapse all 0-rate valves except the starting point
        if to.data.rate == 0 && to.data.name != START {
            self.graph.remove_node(to.data);
            let exits = to
                .exits
                .iter()
                .filter_map(|e| valves.iter().find(|v| e == v.data.name));
            for exit in exits {
                if !visited.contains(&exit.data.name) {
                    visited.push(to.data.name);
                    self.add_tunnel(from, exit, valves, visited);
                }
            }
        } else {
            self.graph.add_node(to.data);
            self.graph.add_edge(*from, to.data, visited.len() as u32);
        }
    }

    fn visit(
        &self,
        visited: Vec<ValveData<'input>>,
        paths: &mut HashMap<Vec<ValveData<'input>>, u32>,
        remaining_time: u32,
        total_time: u32,
    ) -> Vec<(Vec<ValveData<'input>>, u32)> {
        let path_str: Vec<&str> = visited.iter().map(|valve| valve.name).collect();
        // println!("{:?} @ {} minutes", path_str, remaining_time);
        // if we're done visiting stuff, return the score of the path we took
        if total_time == 0 || visited.len() == self.graph.nodes().len() {
            // AA DD BB JJ HH EE CC
            return vec![(visited.to_vec(), self.path_score(&visited, total_time))];
        }

        // visit every node that hasn't been visited already
        let valves_to_visit = self.graph.nodes().filter(|valve| !visited.contains(valve));

        // if there are no more valves to visit, spin down the clock and return

        // if there are valves to visit, visit each one there's time to visit
        let valves_to_visit_at = valves_to_visit.filter_map(|valve| {
            remaining_time
                .checked_sub(self.dist(*visited.last().unwrap(), valve) + 1)
                .map(|arrival_time| (valve, arrival_time))
        });

        for (valve, arrival_time) in valves_to_visit_at {
            let mut new_visited = visited.clone();
            new_visited.push(valve);
            let score = self.path_score(&new_visited, total_time);
            // println!("{} -> {}", score, new_visited.iter().map(|n| n.name).join(", "));
            paths.entry(new_visited.clone()).or_insert(score);

            // paths.(new_visited);
            self.visit(new_visited, paths, arrival_time, total_time);
        }

        // // get the highest score from each of the paths
        // valves_to_visit_at
        //     .map(|(valve, arrival_time)| {
        //         let mut new_visited = visited.clone();
        //         new_visited.push(valve);
        //         self.visit(new_visited, paths, arrival_time, total_time)
        //     })
        //     .for_each(|v| {
        //         paths.extend(v.into_iter());
        //     });

        paths.clone().into_iter().collect_vec()
    }

    fn visit2(
        &self,
        tagged: Vec<ValveData<'input>>,
        activated: Vec<ValveData<'input>>,
        remaining_time: u32,
        total_time: u32,
        players: Vec<Player<'input>>,
        rate: u32,
        pressure_released: u32,
    ) -> (Vec<ValveData>, u32) {

        let mut remaining_time = remaining_time;
        let mut pressure_released = pressure_released;
        let mut rate = rate;
        let mut players = players.clone();
        let mut activated = activated.clone();

        // if both players have nowhere else to go
        if players[0].position.name != START
            && players[0].dest.is_none()
            && players[1].position.name != START
            && players[1].dest.is_none()
        {
            pressure_released += rate * remaining_time;
            return (activated, pressure_released);
        }

        let one_more = activated.len() + 2 == self.graph.nodes().len();

        // move enough minutes forward until one (or both) player(s) reach a valve.

        let minutes = match (players[0].dest, players[1].dest) {
            (Some(_), None) if one_more => players[0].ttm,
            (None, Some(_)) if one_more => players[1].ttm,
            (Some(_), Some(_)) => players[0].ttm.min(players[1].ttm),
            _ => 0,
        };

        // spin time forward for all players to the next valve opening point

        remaining_time -= minutes;
        if players[0].dest.is_some() {
            players[0].ttm -= minutes;
        }
        if players[1].dest.is_some() {
            players[1].ttm -= minutes;
        }

        // open some valves

        pressure_released += rate * minutes;

        // if player 0 arrived at valve _AND_ activated it
        if let Some(dest) = players[0].dest {
            if 0 == players[0].ttm {
                activated.push(dest);
                rate += dest.rate;
                players[0].position = players[0].dest.unwrap();
                players[0].dest = None;
            }
        }

        // if player 1 arrived at valve _AND_ activated it
        if let Some(dest) = players[1].dest {
            if 0 == players[1].ttm {
                activated.push(dest);
                rate += dest.rate;
                players[1].position = players[1].dest.unwrap();
                players[1].dest = None;
            }
        }

        if activated.len() + 1 == self.graph.nodes().len() || remaining_time == 0 {
            pressure_released += rate * remaining_time;
            return (activated, pressure_released);
        }

        // pick a player's index who is ready to move (one of them will be guaranteed to move)

        let moving_player = if players[0].dest.is_none() { 0 } else { 1 };

        // find the valves that have yet to be opened

        let closed_valves = self.graph.nodes().filter(|valve| !tagged.contains(valve));

        // get the highest score from each of the paths

        if let Some(max) = closed_valves
            .map(|valve| {
                let mut tagged = tagged.clone();
                let mut players = players.clone();
                let activated = activated.clone();
                let ttm = self.dist(players[moving_player].position, valve) + 1; // 1 is for activation

                players[moving_player].ttm = ttm;
                players[moving_player].dest = Some(valve); // assign their next location; player
                                                           // will arrive when ttm reaches 0
                tagged.push(valve); // mark the player's next destination as visited so the
                                    // other player doesn't try to go there too
                self.visit2(
                    tagged,
                    activated,
                    remaining_time,
                    total_time,
                    players,
                    rate,
                    pressure_released,
                )
            })
            .max_by(|a, b| {
                a.1.cmp(&b.1)
            })
        {
            println!(
                "{} <- {:?}",
                pressure_released,
                activated.iter().map(|v| v.name).collect::<Vec<&str>>()
            );
            return max;
        } else {
            // pressure_released += rate * remaining_time;
            self.visit2(
                tagged,
                activated,
                remaining_time,
                total_time,
                players,
                rate,
                pressure_released,
            )
        }
    }

    fn path_score(
        &self,
        path: &Vec<ValveData<'input>>,
        // start: ValveData<'input>,
        total_time: u32,
    ) -> u32 {
        let mut time_left = Some(total_time);
        let mut pressure_per_minute = 0;
        let mut score = 0;

        // now that we _just_ opened the first valve, start visiting the rest of the valves;

        for pair in path.windows(2) {
            let from = pair[0];
            let to = pair[1];

            // travel plus activation time
            let minutes = self.dist(from, to) + 1;

            // travel & open the valve
            if let Some(tl) = time_left {
                if tl < minutes {
                    // not enough time left to travel and activate, so just spin down the clock
                    score += pressure_per_minute * tl;
                    break;
                }

                time_left = tl.checked_sub(minutes);
            }

            score += pressure_per_minute * minutes;

            // update the pressure rate to be used in the next iter
            pressure_per_minute += to.rate;

            if time_left.is_none() {
                break;
            }
        }

        // visited everything and time remains?  spin down the clock.
        if let Some(tl) = time_left {
            score += pressure_per_minute * tl;
        }

        score
    }

    /// Get the travel distance from one valve to another.
    fn dist(&self, current: ValveData<'input>, target: ValveData<'input>) -> u32 {
        *self
            .dists
            .as_ref()
            .unwrap()
            .get(&(current, target))
            .expect("distance missing")
    }
}

#[aoc(day16, part1)]
fn part1_solve(input: &str) -> u32 {
    let mut valves = vec![];

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let name = words.nth(1).unwrap();
        let rate = words
            .nth(2)
            .unwrap()
            .replace("rate=", "")
            .replace(';', "")
            .parse::<u32>()
            .unwrap();
        words.nth(3).unwrap(); // "tunnels lead to valve"
        let mut exits = vec![];
        while let Some(word) = words.next() {
            exits.push(word.replace(",", ""));
        }
        let valve = Valve {
            data: ValveData { name, rate, mask: 0 },
            exits,
        };
        valves.push(valve);
    }

    let cave = Cave::new(valves);

    // println!("{:?}", Dot::with_config(&cave.graph, &[]));

    if let Some(start) = cave.start {
        // return cave.scan(start, 30);
        let mut answer = HashMap::new();
        let path_scores = cave.visit(vec![start], &mut answer, 30, 30);
        println!("{:?}", path_scores);
        path_scores.into_iter().map(|p| p.1).max().unwrap()
    } else {
        0
    }
}

#[aoc(day16, part2)]
fn part2_solve(input: &str) -> u32 {
    let mut valves = vec![];

    let mut valve_count = 0;

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let name = words.nth(1).unwrap();
        let rate = words
            .nth(2)
            .unwrap()
            .replace("rate=", "")
            .replace(';', "")
            .parse::<u32>()
            .unwrap();
        words.nth(3).unwrap(); // "tunnels lead to valve"
        let mut exits = vec![];
        while let Some(word) = words.next() {
            exits.push(word.replace(",", ""));
        }

        let valve = Valve {
            data: ValveData { name, rate, mask: 1 << valve_count },
            exits,
        };
        valves.push(valve);

        valve_count += 1;
    }

    let cave = Cave::new(valves);

    // println!("{:?}", Dot::with_config(&cave.graph, &[]));

    if let Some(start) = cave.start {
        // return cave.scan(start, 30);
        let mut paths = HashMap::new();
        let path_scores = cave.visit(vec![start], &mut paths, 26, 26);
        println!("{:#?}", path_scores);

        let mut score = 0;

        for (path1, score1) in &path_scores {
            println!("hi {}", path1.iter().map(|n| n.name).join(", "));
            // for (path2, score2) in &path_scores {
            //     // make sure the paths have no valves in common
            //     let disjoint = path1
            //         .iter()
            //         .filter(|v| v.name != START)
            //         .all(|v| {
            //             !path2.contains(v)
            //         });

            //     if disjoint {
            //         score = score.max(score1 + score2);
            //     }
            //     // for valve in &path1 {
            //     // }
            // }
        }

        score
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_test() {
        // AA DD BB JJ HH EE CC
        assert_eq!(part1_solve(EX), 1651);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solve(EX), 1707);
    }
}
