use aoc_runner_derive::aoc;
use petgraph::algo::floyd_warshall;
use petgraph::prelude::*;
use std::collections::HashMap;

const START: &str = "AA";

#[derive(Debug)]
struct Valve<'name> {
    data: ValveData<'name>,
    exits: Vec<String>,
}

/// The data about a valve that's added to the graph as a node "weight".
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
struct ValveData<'name> {
    name: &'name str,
    rate: u16,
    mask: u16,
}

// impl<'name> std::fmt::Debug for ValveData<'name> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}-{}", self.name, self.rate))
//     }
// }

#[derive(Debug)]
struct Cave<'name> {
    graph: GraphMap<ValveData<'name>, u16, Directed>,
    start: Option<ValveData<'name>>,
    dists: Option<HashMap<(u16, u16), u16>>,
}

impl<'input> Cave<'input> {
    fn new(mut valves: Vec<Valve<'input>>) -> Self {
        let mut cave = Self {
            graph: GraphMap::new(),
            start: None,
            dists: None,
        };

        let mut good_node_count = 0;
        // assign masks to the good nodes
        for valve in valves.iter_mut() {
            if valve.data.rate > 0 || valve.data.name == START {
                (*valve).data.mask = 1 << good_node_count;
                good_node_count += 1;
            }
        }

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
            good_node_count += 1;
        }

        // assign bit masks to valves now that 0-cost valves have been pruned

        // for (i, mut valve) in cave.graph.nodes().enumerate() {
        //     let mask = 1 << i;
        //     println!("{} - {}", valve.name, mask);
        //     valve.mask = mask;
        // }

        println!("{:#?}", cave.graph.nodes());

        // pre-compute distances between each pair of valves

        if let Ok(shortest_paths) = floyd_warshall(&cave.graph, |e| *e.weight()) {
            cave.dists = Some(
                shortest_paths
                    .iter()
                    .map(|path| {
                        if path.1 == &u16::MAX {
                            if let Some(opposite) = shortest_paths.get(&(path.0 .1, path.0 .0)) {
                                return ((path.0 .0.mask, path.0 .1.mask), *opposite);
                            }
                        }
                        return ((path.0 .0.mask, path.0 .1.mask), *path.1);
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
            self.graph.add_edge(*from, to.data, visited.len() as u16);
        }
    }

    fn visit(
        &self,
        visited: u16,
        path: Vec<u16>,
        paths: &'input mut Vec<(u16, u16)>,
        rate: u16,
        score: u16,
        remaining_time: u16,
        total_time: u16,
        all_nodes: u16,
    ) {
        // println!("{} -> {:?}", score, path);

        // if we're done visiting stuff, return the score of the path we took
        // if total_time == 0 || visited == all_nodes {
        //     // AA DD BB JJ HH EE CC
        //     // [1, 8, 2, 512, 128, 16, 4]
        //     return;
        // }

        // if there are valves to visit, visit each one there's time to visit
        // TODO would be nice to refactor this to remove the ValveData struct entirely and use only
        // masks
        let valves_to_visit = self.graph.nodes().filter_map(|valve| {
            if valve.mask & visited > 0 {
                None
            } else {
                remaining_time
                    .checked_sub(self.dist(*path.last().unwrap(), valve.mask) + 1)
                    .map(|arrival_time| (valve, arrival_time))
            }
        });

        for (valve, arrival_time) in valves_to_visit {
            let score = score + rate * (remaining_time - arrival_time);
            let rate = rate + valve.rate;
            let visited = visited | valve.mask;
            let mut path = path.clone();
            path.push(valve.mask);

            // add the score for visiting _only_ this path, so include spinning down remaining time
            // and remove the start node from visited... TODO I should really find a way toremove
            // the start node entirely
            paths.push((
                visited - self.start.unwrap().mask,
                score + rate * arrival_time,
            ));

            self.visit(
                visited,
                path.clone(),
                paths,
                rate,
                score,
                arrival_time,
                total_time,
                all_nodes,
            );
        }
    }

    /// Get the travel distance from one valve to another.
    fn dist(&self, current: u16, target: u16) -> u16 {
        *self
            .dists
            .as_ref()
            .unwrap()
            .get(&(current, target))
            .expect("distance missing")
    }
}

fn generator(input: &str) -> Vec<Valve> {
    let mut valves = vec![];

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let name = words.nth(1).unwrap();
        let rate = words
            .nth(2)
            .unwrap()
            .replace("rate=", "")
            .replace(';', "")
            .parse::<u16>()
            .unwrap();
        words.nth(3).unwrap(); // "tunnels lead to valve"
        let mut exits = vec![];
        while let Some(word) = words.next() {
            exits.push(word.replace(",", ""));
        }

        let valve = Valve {
            data: ValveData {
                name,
                rate,
                mask: 0, // assign this later
            },
            exits,
        };
        valves.push(valve);
    }

    valves
}

#[aoc(day16, part1)]
fn part1_solve(input: &str) -> u16 {
    let valves = generator(input);
    let cave = Cave::new(valves);

    let all_valves_mask = 1 << cave.graph.nodes().len() - 1;
    // println!("{:?}", Dot::with_config(&cave.graph, &[]));

    if let Some(start) = cave.start {
        let mut answer = Vec::new();
        cave.visit(
            start.mask,
            vec![start.mask],
            &mut answer,
            0,
            0,
            30,
            30,
            all_valves_mask,
        );
        answer.into_iter().map(|p| p.1).max().unwrap()
    } else {
        0
    }
}

#[aoc(day16, part2)]
fn part2_solve(input: &str) -> u16 {
    let valves = generator(input);
    let cave = Cave::new(valves);

    let all_valves_mask = (1 << cave.graph.nodes().len()) - 1;

    if let Some(start) = cave.start {
        let mut answer = Vec::new();
        cave.visit(
            start.mask,
            vec![start.mask],
            &mut answer,
            0,
            0,
            26,
            26,
            all_valves_mask,
        );

        let mut score = 0;

        for (visited1, score1) in &answer {
            for (visited2, score2) in &answer {
                if visited1 & visited2 == 0 {
                    score = score.max(score1 + score2);
                }
            }
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
