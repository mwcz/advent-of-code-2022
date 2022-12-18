use aoc_runner_derive::aoc;
use petgraph::algo::{k_shortest_path, dijkstra};
use petgraph::dot::Dot;
use petgraph::prelude::*;
use petgraph::visit::Visitable;

const START: &str = "AA";

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
    graph: UnGraphMap<ValveData<'name>, u16>,
    start: Option<ValveData<'name>>,
}

impl<'input> Cave<'input> {
    fn new(valves: Vec<Valve<'input>>) -> Self {
        let mut cave = Self {
            graph: UnGraphMap::new(),
            start: None,
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

        cave
    }

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
            self.graph.add_edge(*from, to.data, visited.len() as u16);
        }
    }

    fn scan(
        &self,
        start: ValveData<'input>,
        total_time: u16,
    ) /*-> (ValveData<'input>, Vec<ValveData<'input>>, u16) */
    {
        let sum_of_all_nodes = self.graph.nodes().map(|v| v.rate).sum::<u16>();
        let mut closed: Vec<ValveData> = vec![];
        let mut released = 0;
        let mut current: ValveData<'input> = start;
        let mut time_left = total_time;

        struct Visit<'input> {
            /// The valve to visit
            valve: ValveData<'input>,
            /// Cost of traveling to the valve
            cost: u16,
            /// The value of opening the valve based on how much time will be left once you get
            /// there and open it times its rate.
            value: u16,
        }

        // set the initial "best" option as just staying here and doing nothing
        let mut best = Visit {
            valve: start,
            cost: 1,
            value: 0,
        };

        'outer: loop {
            // the rates of all open valves added up
            let opportunity_cost = sum_of_all_nodes - closed.iter().map(|v| v.rate).sum::<u16>();

            if opportunity_cost == 0 {
                // all nodes are open, spin down and exit
                for _ in 0..time_left {
                    released += sum_of_all_nodes;
                    break 'outer;
                }
            }

            let visits = dijkstra(&self.graph, current, None,|e| {
                // the cost of visiting teach node is the distance to it (edge weight) plus the
                // opportunity cost of not opening all the other open nodes
                let weight = 100 * *e.weight();
                let rate = e.target().rate;
                // weight staying in place as a very high cost even
                weight + opportunity_cost - rate
                    // TODO this cost function doesn't adequately weight he cost of travel.
            });

            // evaluate next visit
            for visit in &visits {
                // skip already opened valves or when visiting the current node
                if closed.contains(visit.0) || visit.0 == &current {
                    continue;
                }
                // let cost = (*visit.1 + visit.0.rate) - opportunity_cost;
                let cost = (*visit.1 + visit.0.rate - opportunity_cost) / 100;
                // if we have time to get there _and_ do something (the + 1)
                if let Some(new_time_left) = time_left.checked_sub(cost + 1) {
                    let visit_value = Self::value(visit.0, new_time_left);
                    if visit_value > best.value {
                        best = Visit {
                            valve: *visit.0,
                            cost,
                            value: visit_value,
                        };
                    }
                }
            }

            // all potential visits evaluated, now visit the best one
            println!("move from {:?} to {:?} and open it ({} minutes)", current, best.valve, best.cost + 1);
            current = best.valve;
            time_left -= best.cost + 1; // cost plus opening time
            closed.push(current);

            // update released pressure corresponding to travel time
            released += (best.cost+1) * closed.iter().map(|v| v.rate).sum::<u16>();

            // reset cost and value so once we've visited everything and can't move anymore, we
            // don't get stuck updating the time with an old cost
            best.cost = 1;
            best.value = 0;

            if time_left == 0 {
                break;
            }
        }

        println!("{:#?}", released);
    }

    // fn best_move(&self, node: ValveData<'input>) -> ValveData<'input> {
    //     // look up to 5 nodes away and add up
    // }

    /// How much pressure will be released by this node if the open command is issued when the
    /// given amount of time is remaining.
    fn value(node: &ValveData<'input>, time_left: u16) -> u16 {
        node.rate * (time_left - 1)
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

    // println!("{:?}", Dot::with_config(&cave.graph, &[]));

    if let Some(start) = cave.start {
        cave.scan(start, 30);
    }

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
