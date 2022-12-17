use aoc_runner_derive::aoc;
use petgraph::{prelude::*, dot::{Dot, Config}};

#[derive(Debug)]
struct Cave<'name> {
    graph: DiGraphMap<ValveData<'name>, u16>,
}

impl<'input> Cave<'input> {
    fn new(valves: Vec<Valve<'input>>) -> Self {
        let mut cave = Self {
            graph: DiGraphMap::new(),
        };

        // add valve nodes

        for valve in &valves {
            let exits = valve.exits.iter().filter_map(|e| valves.iter().find(|v| e == v.data.name));
            for exit in exits {
                cave.add_tunnel(&valve.data, &exit.data);
            }
        }

        cave
    }

    fn add_tunnel(&mut self, from: &ValveData<'input>, to: &ValveData<'input>) {
        self.graph.add_node(*from);
        self.graph.add_node(*to);

        self.graph.add_edge(*from, *to, 1);
    }

    // fn add_valve(&mut self, valve: Valve<'input>) {
    //     self.graph.add_node(valve);
    // }

    // fn add_tunnel(&mut self, valve_idx: NodeIndex, exit_idx: NodeIndex, cost: u16) -> EdgeIndex {
    //     self.graph.add_edge(valve_idx, exit_idx, cost)
    // }

    // fn get_valve_by_name(&self, name: &'input str) -> &Valve {
    //     self.graph
    //         .node_weights()
    //         .find(|weight| weight.name == name)
    //         .unwrap()
    // }

    // fn get_node_by_name(&self, name: &'input str) -> NodeIndex {
    //     self.graph
    //         .node_indices()
    //         .find(|idx| self.graph[*idx].name == name)
    //         .unwrap()
    // }

    // fn get_valve(&self, idx: NodeIndex) -> Option<&Valve<'input>> {
    //     self.graph.node_weight(idx)
    // }

    // fn dig_tunnels(&mut self) {
    //     // add edges

    //     let mut tunnels: Vec<(NodeIndex, NodeIndex)> = vec![];

    //     for valve in self.graph.node_weights() {
    //         let valve_idx = self.get_node_by_name(valve.name);
    //         if let Some(exits) = &valve.exits {
    //             for exit in exits {
    //                 tunnels.push((valve_idx, *exit));
    //             }
    //         }
    //     }
    //     for tunnel in tunnels {
    //         self.add_tunnel(tunnel.0, tunnel.1, 1);
    //     }
    // }
}

#[derive(Debug)]
struct Valve<'name> {
    data: ValveData<'name>,
    exits: Vec<String>,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
struct ValveData<'name> {
    name: &'name str,
    rate: u16,
}

impl<'name> std::fmt::Debug for ValveData<'name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.name, self.rate))
    }
}

#[aoc(day16, part1)]
fn part1_solve(input: &str) -> usize {
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
            data: ValveData { name, rate },
            exits,
        };
        valves.push(valve);
    }

    let cave = Cave::new(valves);

    println!("{:?}", Dot::with_config(&cave.graph, &[Config::EdgeNoLabel]));

    todo!();
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
        assert_eq!(part1_solve(EX), 26);
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(part2_solve(EX, 20), 56000011);
    // }
}
