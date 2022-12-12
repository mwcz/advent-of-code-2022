use std::ops::{Add, Mul};
use num_bigint::BigUint;

use aoc_runner_derive::aoc;

struct Monkey {
    items: Vec<Item>,
    op: Operation,
    test: Test,
    items_seen: BigUint,
}

struct Operation {
    op: fn(BigUint, BigUint) -> BigUint,
    /// None implies "old"
    value: Option<BigUint>,
}
#[derive(Clone)]
struct Item {
    worry: BigUint,
}
struct Test {
    div: BigUint,
    if_true: usize,
    if_false: usize,
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        lines.next().unwrap(); // discard monkey label

        let items: Vec<Item> = lines
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .split(",")
            .map(|n| Item {
                worry: n.trim().parse::<BigUint>().unwrap(),
            })
            .collect();

        let mut op = lines.next().unwrap().split_whitespace().rev();

        let op_val = match op.next().unwrap() {
            "old" => None,
            n => n.parse::<BigUint>().ok(),
        };
        let op_op = match op.next().unwrap() {
            "*" => BigUint::mul,
            _ => BigUint::add,
        };

        let div = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<BigUint>()
            .unwrap();

        let if_true = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let if_false = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            op: Operation {
                op: op_op,
                value: op_val,
            },
            test: Test {
                div,
                if_true,
                if_false,
            },
            items_seen: BigUint::from(0u8),
        }
    }
}

#[aoc(day11, part1)]
fn part1_solve(input: &str) -> BigUint {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|line| Monkey::from(line)).collect();
    let mut airborne: Vec<Vec<Item>> = vec![vec![]; monkeys.len()];

    for _ in 1..=20 {
        for (monkey_idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.append(&mut airborne[monkey_idx]);
            for _ in 0..monkey.items.len() {
                let mut item = monkey.items.remove(0);
                monkey.items_seen += BigUint::from(1u8);
                item.worry = (monkey.op.op)(item.worry.clone(), monkey.op.value.as_ref().unwrap_or(&item.worry).clone()) / BigUint::from(3u8);
                if &item.worry % &monkey.test.div == BigUint::from(0u8) {
                    airborne[monkey.test.if_true].push(item.clone());
                } else {
                    airborne[monkey.test.if_false].push(item);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_seen.cmp(&a.items_seen));

    monkeys[0..=1]
        .iter()
        .map(|monkey| monkey.items_seen.clone())
        .product()
}

#[aoc(day11, part2)]
fn part2_solve(input: &str) -> BigUint {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|line| Monkey::from(line)).collect();
    let mut airborne: Vec<Vec<Item>> = vec![vec![]; monkeys.len()];

    for _round in 1..=10000 {
        for (monkey_idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.append(&mut airborne[monkey_idx]);
            for _ in 0..monkey.items.len() {
                let mut item = monkey.items.remove(0);
                monkey.items_seen += BigUint::from(1u8);
                item.worry = (monkey.op.op)(item.worry.clone(), monkey.op.value.as_ref().unwrap_or(&item.worry).clone());
                if &item.worry % &monkey.test.div == BigUint::from(0u8) {
                    airborne[monkey.test.if_true].push(item);
                } else {
                    airborne[monkey.test.if_false].push(item);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_seen.cmp(&a.items_seen));

    monkeys[0..=1]
        .iter()
        .map(|monkey| monkey.items_seen.clone())
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    #[test]
    fn part1_test() {
        assert_eq!(part1_solve(EX), BigUint::from(10605u32));
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solve(EX), BigUint::from(2713310158u64));
    }
}
