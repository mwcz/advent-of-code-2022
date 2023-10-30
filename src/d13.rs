use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

type Parsed = String;

pub fn parse(input: String) -> Parsed {
    input
}

pub fn part1(input: Parsed) -> usize {
    let packets: Vec<Packet> = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                serde_json::from_str::<Data>(line)
                    .ok()
                    .map(|data| Packet { data, tag: None })
            }
        })
        .collect();

    let correct = packets.chunks(2).enumerate().filter_map(|(i, chunk)| {
        if let Ordering::Less = chunk[0].data.cmp(&chunk[1].data) {
            Some(i + 1)
        } else {
            None
        }
    });

    correct.sum()
}

pub fn part2(input: Parsed) -> usize {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                serde_json::from_str::<Data>(line)
                    .ok()
                    .map(|data| Packet { data, tag: None })
            }
        })
        .collect();

    // add divider packets
    let div2 = Packet {
        data: Data::List(vec![Data::List(vec![Data::Int(2)])]),
        tag: Some(2),
    };
    let div6 = Packet {
        data: Data::List(vec![Data::List(vec![Data::Int(6)])]),
        tag: Some(6),
    };
    packets.push(div2);
    packets.push(div6);

    packets.sort();

    (1 + packets.iter().position(|p| p.tag == Some(2)).unwrap())
        * (1 + packets.iter().position(|p| p.tag == Some(6)).unwrap())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Packet {
    data: Data,
    tag: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(untagged)]
enum Data {
    Int(u8),
    List(Vec<Data>),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Data::Int(s), Data::Int(o)) => s.cmp(o),
            (Data::Int(s), Data::List(_)) => Data::List(vec![Data::Int(*s)]).cmp(other),
            (Data::List(_), Data::Int(o)) => self.cmp(&Data::List(vec![Data::Int(*o)])),
            (Data::List(s), Data::List(o)) => {
                let mut si = s.iter();
                let mut oi = o.iter();
                loop {
                    match (si.next(), oi.next()) {
                        (None, None) => {
                            return Ordering::Equal;
                        }
                        (None, Some(_)) => {
                            return Ordering::Less;
                        }
                        (Some(_), None) => {
                            return Ordering::Greater;
                        }
                        (Some(a), Some(b)) => {
                            match a.cmp(b) {
                                Ordering::Less => return Ordering::Less,
                                Ordering::Greater => return Ordering::Greater,
                                Ordering::Equal => { /* continue */ }
                            }
                        }
                    }
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const EX: &str = "[1,1,3,1,1]
// [1,1,5,1,1]
//
// [[1],[2,3,4]]
// [[1],4]
//
// [9]
// [[8,7,6]]
//
// [[4,4],4,4]
// [[4,4],4,4,4]
//
// [7,7,7,7]
// [7,7,7]
//
// []
// [3]
//
// [[[]]]
// [[]]
//
// [1,[2,[3,[4,[5,6,7]]]],8,9]
// [1,[2,[3,[4,[5,6,0]]]],8,9]";
//
//     #[test]
//     fn part1_test() {
//         assert_eq!(part1_solve(EX), 13);
//     }
//
//     #[test]
//     fn part2_test() {
//         assert_eq!(part2_solve(EX), 140);
//     }
// }
