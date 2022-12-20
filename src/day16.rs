use aoc_runner_derive::aoc;
use itertools::Itertools;
use petgraph::algo::{dijkstra, floyd_warshall};
use petgraph::dot::Dot;
use petgraph::prelude::*;
use std::collections::HashMap;

const START: &str = "AA";

#[derive(Debug)]
struct Valve<'name> {
    data: ValveData<'name>,
    exits: Vec<String>,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
struct ValveData<'name> {
    name: &'name str,
    rate: u32,
}

impl<'name> std::fmt::Debug for ValveData<'name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.name, self.rate))
    }
}

#[derive(Debug)]
struct Cave<'name> {
    graph: DiGraphMap<ValveData<'name>, u32>,
    start: Option<ValveData<'name>>,
    dists: Option<HashMap<(ValveData<'name>, ValveData<'name>), u32>>,
    i: usize,
}

impl<'input> Cave<'input> {
    fn new(valves: Vec<Valve<'input>>) -> Self {
        let mut cave = Self {
            graph: DiGraphMap::new(),
            start: None,
            dists: None,
            i: 0,
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
            // node
            // was fun but disabling for now because with the dists already available it doesn't
            // really help to have them built into the graph edges.

            // for ((a, b), dist) in shortest_paths.iter() {
            //     cave.graph.remove_edge(*a, *b);
            //     if a != b {
            //         cave.graph.add_edge(*a, *b, *dist);
            //     }
            // }
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

    fn scan(&self, start: ValveData<'input>, total_time: u32) -> u32 {
        let paths = self
            .graph
            .nodes()
            .filter(|valve| {
                valve.name != start.name
            })
            .permutations(self.graph.node_count() - 1 /* since start is omitted */);

        // for path in paths {
        //     println!("{:?} -> {}", path, self.path_score(&path, start, total_time));
        // }

        paths.map(|path| self.path_score(&path, start, total_time)).max().unwrap_or(0)
    }

    fn path_score(&self, path: &Vec<ValveData<'input>>, start: ValveData<'input>, total_time: u32) -> u32 {
        // initialize these starting values by "visiting" the first valve in the path
        let path_str: Vec<&str> = path.iter().map(|valve| valve.name).collect();
        let mut time_left = Some(total_time - self.dist(start, *path.first().unwrap()) - 1);
        let mut pressure_per_minute = path.first().unwrap().rate;
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

    // fn scan(&mut self, start: ValveData<'input>, total_time: u32)
    // /*-> (ValveData<'input>, Vec<ValveData<'input>>, u32) */
    // {
    //     // let sum_of_all_rates = self.graph.nodes().map(|v| v.rate).sum::<u32>();
    //     let mut released = 0;
    //     let mut current: ValveData<'input> = start;
    //     let mut time_left = total_time;

    //     struct Visit<'input> {
    //         /// The valve to visit
    //         valve: ValveData<'input>,
    //         /// Cost of traveling to the valve
    //         cost: u32,
    //         value: u32,
    //     }

    //     // set the initial "best" option as just staying here and doing nothing
    //     let mut best = Visit {
    //         valve: start,
    //         cost: 1,
    //         value: u32::MAX,
    //     };

    //     loop {
    //         // if all valves are opened, spin down the remaining time
    //         if self.opened.len() == self.graph.nodes().len() {
    //             released += time_left * self.opened.iter().map(|v| v.rate).sum::<u32>();
    //             break;
    //         }

    //         static DIJKSTRA_OFFSET: u32 = u32::MAX/3;

    //         let visits = dijkstra(&self.graph, current, None, |e| {
    //             DIJKSTRA_OFFSET - self.value(e.source(), e.target(), time_left).unwrap_or(0)
    //         });

    //         // evaluate next visit
    //         for visit in &visits {
    //             // don't consider already opened valves or when visiting the current valve
    //             // if self.opened.contains(visit.0) || visit.0 == &current {
    //             //     continue;
    //             // }

    //             let cost = *self
    //                 .dists
    //                 .as_ref()
    //                 .unwrap()
    //                 .get(&(current, *visit.0))
    //                 .unwrap();

    //             if let Some(new_time_left) = time_left.checked_sub(cost + 1) {
    //                 // recover the value from the dijkstrafied value
    //                 let recovered_value = DIJKSTRA_OFFSET - *visit.1;
    //                 if recovered_value > best.value {
    //                     best = Visit {
    //                         valve: *visit.0,
    //                         cost,
    //                         value: recovered_value,
    //                     };
    //                 }
    //             }
    //         }

    //         // all potential visits evaluated, now visit the best one
    //         println!(
    //             "move from {:?} to {:?} and open it ({} minutes)",
    //             current,
    //             best.valve,
    //             best.cost + 1
    //         );
    //         current = best.valve;

    //         // subtract travel cost plus opening time
    //         let Some(new_time_left) = time_left.checked_sub(best.cost + 1) else {
    //             break;
    //         };
    //         time_left = new_time_left;

    //         self.opened.push(current);

    //         // update released pressure corresponding to travel time
    //         released += (best.cost + 1) * self.opened.iter().map(|v| v.rate).sum::<u32>();

    //         // reset cost and value so once we've visited everything and can't move anymore, we
    //         // don't get stuck updating the time with an old cost
    //         best.cost = 1;
    //         best.value = u32::MAX;

    //         if time_left == 0 {
    //             break;
    //         }
    //     }

    //     println!("{:#?}", released);
    // }

    // A heuristic for how much value can be gained, starting at the given time, by moving from
    // current valve to target valve and activating the target valve.
    fn value(
        &self,
        current: ValveData<'input>,
        target: ValveData<'input>,
        time_left: u32,
    ) -> Option<u32> {
        let travel_time = self.dist(current, target);
        let potential = self.release_potential_with_travel(current, travel_time, time_left);
        let proximate_potential = self.proximate_values(target, travel_time, time_left);
        Some(potential + proximate_potential)
    }

    /// How much pressure will be released by this valve if the open command is issued when the
    /// given amount of time is remaining.
    fn release_potential(&self, valve: ValveData<'input>, time_left: u32) -> u32 {
        // subtract 1 minute for opening the valve, then compute pressure release
        valve.rate * time_left.checked_sub(1).unwrap_or(0)
    }

    /// If we're at the given current valve, and at `time_left` we decide to travel to the given
    /// target valve and activate it, how much pressure will be relased by the end of time?
    fn release_potential_with_travel(
        &self,
        target: ValveData<'input>,
        travel_time: u32,
        time_left: u32,
    ) -> u32 {
        let time_left_at_arrival = time_left.checked_sub(travel_time).unwrap_or(0);
        self.release_potential(target, time_left_at_arrival)
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

    /// A heuristic estimating how much value a given valve has due only to its proximity to other
    /// high-value (ie, high pressure-release potential) valves.
    fn proximate_values(&self, valve: ValveData<'input>, travel_time: u32, time_left: u32) -> u32 {
        self.graph
            .nodes()
            .filter_map(|other_valve| {
                // consider only valves that are still closed
                if self.opened.contains(&other_valve) {
                    None
                } else {
                    // TODO this is recursive for now... put a limit on recursion if it's too slow
                    Some(self.release_potential_with_travel(other_valve, travel_time, time_left))
                }
            })
            .sum::<u32>()
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
            data: ValveData { name, rate },
            exits,
        };
        valves.push(valve);
    }

    let cave = Cave::new(valves);

    // println!("{:?}", Dot::with_config(&cave.graph, &[]));

    if let Some(start) = cave.start {
        return cave.scan(start, 30);
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

    // #[test]
    // fn part2_test() {
    //     assert_eq!(part2_solve(EX, 20), 56000011);
    // }
}
