use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet, VecDeque};

type Name<'a> = &'a str;
type Operator = char;
#[derive(Debug, Clone, Copy)]
enum Value<'a> {
    Resolved((Name<'a>, i128)),
    Pending((Name<'a>, Formula<'a>)),
}
type Formula<'a> = (Name<'a>, Operator, Name<'a>);

impl<'a> Value<'a> {
    fn name(&self) -> Name<'a> {
        match self {
            Value::Resolved(a) => a.0,
            Value::Pending(a) => a.0,
        }
    }
    fn is_resolved(&self) -> bool {
        if let Value::Resolved(_) = self {
            true
        } else {
            false
        }
    }
    fn is_pending(&self) -> bool {
        if let Value::Pending(_) = self {
            true
        } else {
            false
        }
    }
}

fn part1_solve(input: &str) -> i128 {
    let mut vals: HashMap<Name, Value> = HashMap::new();
    let values: Vec<Vec<String>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.replace(':', ""))
                .collect::<Vec<String>>()
        })
        .collect();

    for strs in values.iter() {
        if strs.len() == 2 {
            // already resolved
            vals.insert(
                &strs[0],
                Value::Resolved((&strs[0], strs[1].parse::<i128>().unwrap())),
            );
        } else {
            // formula
            vals.insert(
                &strs[0],
                Value::Pending((
                    &strs[0],
                    (&strs[1], strs[2].chars().next().unwrap(), &strs[3]),
                )),
            );
        }
    }

    let root: Value = *vals.get("root").unwrap();

    let mut q = VecDeque::from([root]);

    while !q.is_empty() {
        let next = *q.back().unwrap();

        match next {
            Value::Resolved(_) => {
                // resolved; remove it from the queue
                q.pop_back();
            }
            Value::Pending((name, (a, op, b))) => {
                let a = vals.get(a).unwrap();
                let b = vals.get(b).unwrap();

                if a.is_pending() {
                    q.push_back(*a);
                }

                if b.is_pending() {
                    q.push_back(*b);
                }

                // if both sub-expressions are resolved, compute them and resolve this expression
                if let (Value::Resolved((_, aval)), Value::Resolved((_, bval))) = (a, b) {
                    // both parts of the formula are resolved, so make the current Value
                    // resolved as well
                    let newval = match op {
                        '/' => aval / bval,
                        '*' => aval * bval,
                        '+' => aval + bval,
                        '-' => aval - bval,
                        _ => unreachable!(),
                    };
                    vals.insert(name, Value::Resolved((name, newval)));
                    q.pop_back();
                }
            }
        }
    }

    // println!("{:#?}", vals);

    let Value::Resolved(ans) = vals.get("root").unwrap() else {
        panic!("root still unresolved!");
    };

    ans.1
}

#[aoc(day21, part1)]
fn part1_solver(input: &str) -> i128 {
    part1_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    const INPUT: &str = include_str!("../input/2022/day21.txt");

    #[test]
    fn day21_part1_test() {
        assert_eq!(part1_solve(EX), 152);
    }

    #[test]
    fn day21_part1_real() {
        assert_eq!(part1_solve(INPUT), 194058098264286);
    }
}
