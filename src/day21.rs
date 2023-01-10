use aoc_runner_derive::aoc;
use std::collections::{HashMap, VecDeque};

type Name<'a> = &'a str;
type Operator = char;
#[derive(Debug, Clone, Copy)]
enum Value<'a> {
    Unresolvable(Name<'a>),
    Resolved((Name<'a>, i128)),
    Pending((Name<'a>, Formula<'a>)),
}
type Formula<'a> = (Name<'a>, Operator, Name<'a>);

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

    let mut que = VecDeque::from([root]);

    println!("{:#?}", que);

    while !que.is_empty() {
        let next = que.back().unwrap();

        match next {
            Value::Resolved(_) => {
                unreachable!("resolved value should not enter the queue");
            }
            Value::Pending((name, (a, op, b))) => {
                let a = vals.get(*a).unwrap();
                let b = vals.get(*b).unwrap();

                match (a, b) {
                    (Value::Resolved((_, aval)), Value::Resolved((_, bval))) => {
                        // both parts of the formula are resolved, so make the current Value
                        // resolved as well
                        let newval = match op {
                            '/' => aval / bval,
                            '*' => aval * bval,
                            '+' => aval + bval,
                            '-' => aval - bval,
                            '=' => {
                                println!("root: {a:?} = {b:?}",);
                                0
                            }
                            _ => unreachable!(),
                        };
                        vals.insert(name, Value::Resolved((name, newval)));
                        que.pop_back();
                    }
                    (Value::Resolved(_), Value::Pending(_)) => {
                        que.push_back(*b);
                    }
                    (Value::Pending(_), Value::Resolved(_)) => {
                        que.push_back(*a);
                    }
                    (Value::Pending(_), Value::Pending(_)) => {
                        que.push_back(*a);
                        que.push_back(*b);
                    }
                    // unresolvable isn't used in part1
                    (Value::Unresolvable(_), Value::Unresolvable(_)) => panic!(),
                    (Value::Unresolvable(_), Value::Resolved(_)) => panic!(),
                    (Value::Unresolvable(_), Value::Pending(_)) => panic!(),
                    (Value::Resolved(_), Value::Unresolvable(_)) => panic!(),
                    (Value::Pending(_), Value::Unresolvable(_)) => panic!(),
                }
            }
            Value::Unresolvable(_) => {
                unreachable!("unresolvable value should not enter the queue");
            }
        }
    }

    // println!("{:#?}", vals);

    let Value::Resolved(ans) = vals.get("root").unwrap() else {
        panic!("root still unresolved!");
    };

    ans.1
}

fn part2_solve(input: &str) -> i128 {
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
            if &strs[0] == "humn" {
                vals.insert("humn", Value::Unresolvable("humn"));
            } else {
                vals.insert(
                    &strs[0],
                    Value::Pending((
                        &strs[0],
                        (
                            &strs[1],
                            if &strs[0] == "root" {
                                '='
                            } else {
                                strs[2].chars().next().unwrap()
                            },
                            &strs[3],
                        ),
                    )),
                );
            }
        }
    }

    let root: Value = *vals.get("root").unwrap();

    // first operand of root's formula
    let Value::Pending((_, (a, _op, b))) = root else {
        panic!("root must start pending");
    };
    let root_a = a;
    let root_b = b;

    let que_a = VecDeque::from([*vals.get(root_a).unwrap()]);
    let que_b = VecDeque::from([*vals.get(root_b).unwrap()]);

    fn solve<'a>(
        mut que: VecDeque<Value<'a>>,
        mut vals: HashMap<Name<'a>, Value<'a>>,
        root: &str,
    ) -> Option<i128> {
        while !que.is_empty() {
            let next = que.back().unwrap();

            if let Value::Resolved((_, num)) = vals.get(root).unwrap() {
                return Some(*num);
            }

            match next {
                Value::Resolved(_) => {
                    unreachable!("resolved value should not enter the queue");
                }
                Value::Pending((name, (a, op, b))) => {
                    let a = vals.get(*a).unwrap();
                    let b = vals.get(*b).unwrap();

                    match (a, b) {
                        (Value::Resolved((_, aval)), Value::Resolved((_, bval))) => {
                            // both parts of the formula are resolved, so make the current Value
                            // resolved as well
                            let newval = match op {
                                '/' => aval / bval,
                                '*' => aval * bval,
                                '+' => aval + bval,
                                '-' => aval - bval,
                                '=' => continue,
                                _ => unreachable!(),
                            };

                            if *name == root {
                                return Some(newval);
                            }

                            vals.insert(name, Value::Resolved((name, newval)));
                            que.pop_back();
                        }
                        (Value::Resolved(_), Value::Pending(_)) => {
                            que.push_back(*b);
                        }
                        (Value::Pending(_), Value::Resolved(_)) => {
                            que.push_back(*a);
                        }
                        (Value::Pending(_), Value::Pending(_)) => {
                            que.push_back(*a);
                            que.push_back(*b);
                        }
                        (Value::Unresolvable(_), Value::Unresolvable(_)) => {
                            panic!("can't have two unresolvable operands");
                        }
                        (Value::Unresolvable(_), Value::Resolved(_))
                        | (Value::Resolved(_), Value::Unresolvable(_))
                        | (Value::Pending(_), Value::Unresolvable(_))
                        | (Value::Unresolvable(_), Value::Pending(_)) => {
                            // unresolvable == skip this branch
                            return None;
                        }
                    }
                }
                Value::Unresolvable(_) => {
                    unreachable!("unresolvable value should not enter the queue");
                }
            }
        }
        Some(0)
    }

    solve(que_a, vals.clone(), root_a)
        .unwrap_or_else(|| solve(que_b, vals.clone(), root_b).unwrap())
}

#[aoc(day21, part1)]
fn part1_solver(input: &str) -> i128 {
    part1_solve(input)
}

#[aoc(day21, part2)]
fn part2_solver(input: &str) -> i128 {
    part2_solve(input)
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

    #[test]
    fn day21_part1_test() {
        assert_eq!(part1_solve(EX), 152);
    }

    #[test]
    fn day21_part2_test() {
        assert_eq!(part2_solve(EX), 301);
    }
}
