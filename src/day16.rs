use aoc_runner_derive::aoc;
use petgraph::{
    algo::floyd_warshall,
    dot::{Config, Dot},
    prelude::*,
};

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
            if valve.data.name != "AA" && valve.data.rate == 0 {
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

        cave
    }

    fn add_tunnel(&mut self, from: &ValveData<'input>, to: &Valve<'input>, valves: &Vec<Valve<'input>>, visited: &mut Vec<&'input str>) {
        self.graph.add_node(*from);

        const START: &str = "AA";

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

    println!("{:?}", Dot::with_config(&cave.graph, &[]));

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
        assert_eq!(part1_solve(EX), 1651);
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(part2_solve(EX, 20), 56000011);
    // }
}
