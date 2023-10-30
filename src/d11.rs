type Parsed = String;

pub fn parse(input: String) -> Parsed {
    input
}

pub fn part1(input: Parsed) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
    let mut airborne: Vec<Vec<Item>> = vec![vec![]; monkeys.len()];

    let three = 3;

    for _ in 1..=20 {
        for (monkey_idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.append(&mut airborne[monkey_idx]);
            for _ in 0..monkey.items.len() {
                let mut item = monkey.items.remove(0);
                monkey.items_seen += 1;
                item.worry = match monkey.op.op {
                    MathOp::Mul => match &monkey.op.value {
                        Some(val) => item.worry * val,
                        None => item.worry * item.worry,
                    },
                    MathOp::Add => match &monkey.op.value {
                        Some(val) => item.worry + val,
                        None => unreachable!(),
                    },
                };

                item.worry /= &three;

                if item.worry % monkey.test.div == 0 {
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
        .map(|monkey| &monkey.items_seen)
        .product()
}

pub fn part2(input: Parsed) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
    let mut airborne: Vec<Vec<Item>> = vec![vec![]; monkeys.len()];

    let max: u64 = monkeys.iter().map(|m| &m.test.div).product();

    for _round in 1..=10000 {
        for (monkey_idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.append(&mut airborne[monkey_idx]);
            for _ in 0..monkey.items.len() {
                let mut item = monkey.items.remove(0);
                monkey.items_seen += 1;
                item.worry = match monkey.op.op {
                    MathOp::Mul => match &monkey.op.value {
                        Some(val) => item.worry * val,
                        None => item.worry * item.worry,
                    },
                    MathOp::Add => match &monkey.op.value {
                        Some(val) => item.worry + val,
                        None => {
                            unreachable!();
                        }
                    },
                };

                item.worry %= &max;

                if item.worry % monkey.test.div == 0 {
                    // item.worry /= &monkey.test.div;
                    airborne[monkey.test.if_true].push(item);
                } else {
                    // item.worry %= &monkey.test.div;
                    airborne[monkey.test.if_false].push(item);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_seen.cmp(&a.items_seen));

    monkeys[0..=1]
        .iter()
        .map(|monkey| &monkey.items_seen)
        .product()
}

struct Monkey {
    items: Vec<Item>,
    op: Operation,
    test: Test,
    items_seen: u64,
}

enum MathOp {
    Mul,
    Add,
}

struct Operation {
    op: MathOp,
    /// None implies "old"
    value: Option<u64>,
}
#[derive(Clone)]
struct Item {
    worry: u64,
}
struct Test {
    div: u64,
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
            .split(':')
            .last()
            .unwrap()
            .split(',')
            .map(|n| Item {
                worry: n.trim().parse::<u64>().unwrap(),
            })
            .collect();

        let mut op = lines.next().unwrap().split_whitespace().rev();

        let op_val = match op.next().unwrap() {
            "old" => None,
            n => n.parse::<u64>().ok(),
        };
        let op_op = match op.next().unwrap() {
            "*" => MathOp::Mul,
            _ => MathOp::Add,
        };

        let div = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
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
            items: items.into_iter().collect(),
            op: Operation {
                op: op_op,
                value: op_val,
            },
            test: Test {
                div,
                if_true,
                if_false,
            },
            items_seen: 0,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const EX: &str = "Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
//
// Monkey 1:
//   Starting items: 54, 65, 75, 74
//   Operation: new = old + 6
//   Test: divisible by 19
//     If true: throw to monkey 2
//     If false: throw to monkey 0
//
// Monkey 2:
//   Starting items: 79, 60, 97
//   Operation: new = old * old
//   Test: divisible by 13
//     If true: throw to monkey 1
//     If false: throw to monkey 3
//
// Monkey 3:
//   Starting items: 74
//   Operation: new = old + 3
//   Test: divisible by 17
//     If true: throw to monkey 0
//     If false: throw to monkey 1";
//     #[test]
//     fn part1_test() {
//         assert_eq!(part1_solve(EX), 10605u64);
//     }
//
//     #[test]
//     fn part2_test() {
//         assert_eq!(part2_solve(EX), 2713310158u64);
//     }
// }
