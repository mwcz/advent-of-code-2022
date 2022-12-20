use aoc_runner_derive::aoc;
use itertools::Itertools;
use petgraph::algo::floyd_warshall;
use petgraph::prelude::*;
use rayon::prelude::*;
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
            .filter(|valve| valve.name != start.name)
            .permutations(
                self.graph.node_count() - 1, /* since start is omitted */
            )
            .par_bridge();

        paths
            .map(|path| self.path_score(&path, start, total_time))
            .max()
            .unwrap_or(0)
    }

    fn path_score(
        &self,
        path: &Vec<ValveData<'input>>,
        start: ValveData<'input>,
        total_time: u32,
    ) -> u32 {
        // initialize these starting values by "visiting" the first valve in the path
        let path_str: Vec<&str> = path.iter().map(|valve| valve.name).collect();
        println!("{:?}", path_str);
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
