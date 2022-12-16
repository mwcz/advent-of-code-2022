use aoc_runner_derive::aoc;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(Debug, PartialEq, Eq)]
struct Valve<'name> {
    name: &'name str,
    rate: u16,
    exits: Vec<&'name str>,
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
            exits.push(word);
        }
        let valve = Valve { name, rate, exits };
        valves.push(valve);
    }

    let mut graph = Graph::new();

    for 

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
