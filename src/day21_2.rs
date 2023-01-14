use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet, VecDeque};

type Name<'a> = &'a str;
type Operator = char;
#[derive(Debug, Clone, Copy)]
enum Value<'a> {
    Symbol(Name<'a>),
    Resolved((Name<'a>, i128)),
    Pending((Name<'a>, Formula<'a>)),
}
type Formula<'a> = (Name<'a>, Operator, Name<'a>);

impl<'a> Value<'a> {
    fn name(&self) -> Name<'a> {
        match self {
            Value::Symbol(a) => *a,
            Value::Resolved(a) => a.0,
            Value::Pending(a) => a.0,
        }
    }
}

fn part2_solve(input: &str) -> i128 {
    let mut vals: HashMap<Name, Value> = HashMap::new();
    let mut resolved_eqs: HashSet<Name> = HashSet::new();
    // The parts of the final equation
    let mut eq: Vec<Name> = Vec::new();
    let values: Vec<Vec<String>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.replace(':', ""))
                .collect::<Vec<String>>()
        })
        .collect();

    for strs in values.iter() {
        if &strs[0] == "humn" {
            vals.insert("humn", Value::Symbol("humn"));
        } else {
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

    let mut que_a = VecDeque::from([*vals.get(root_a).unwrap()]);
    let mut que_b = VecDeque::from([*vals.get(root_b).unwrap()]);

    fn solve<'a>(
        que: &mut VecDeque<Value<'a>>,
        vals: &mut HashMap<Name<'a>, Value<'a>>,
        resolved_eqs: &mut HashSet<Name<'a>>,
        eqs: &mut Vec<Name<'a>>,
        start: &str,
    ) -> Option<i128> {
        let mut done = false;
        while !done {
            let next = que.back().unwrap();

            if let Value::Resolved((_, num)) = vals.get(start).unwrap() {
                return Some(*num);
            }

            if resolved_eqs.contains(next.name()) {
                eqs.push(next.name());
                que.pop_back();
            } else {
                match next {
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

                                if *name == start {
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
                            (Value::Symbol(_), Value::Resolved(_)) => {
                                resolved_eqs.insert(name);
                                que.pop_back();
                            }
                            (Value::Resolved(_), Value::Symbol(_)) => {
                                resolved_eqs.insert(name);
                                que.pop_back();
                            }
                            (Value::Pending(_), Value::Symbol(_)) => {
                                que.push_back(*a);
                            }
                            (Value::Symbol(_), Value::Pending(_)) => {
                                que.push_back(*b);
                            }
                            (Value::Symbol(_), Value::Symbol(_)) => {
                                panic!("can't have two unresolvable operands");
                            }
                        }
                    }
                    Value::Symbol(_) => {
                        unreachable!("resolved value should not enter the queue");
                    }
                    Value::Resolved(_) => {
                        unreachable!("resolved value should not enter the queue");
                    }
                }
            }
            println!("que length: {}", que.len());
            // println!("que {que:#?}", );
            println!(
                "{:?}",
                que.iter()
                    .map(|n| {
                        match n {
                            Value::Symbol(a) => *a,
                            Value::Resolved(a) => a.0,
                            Value::Pending(a) => a.0,
                        }
                    })
                    .collect::<Vec<&str>>()
            );
        }
        Some(0)
    }

    let a_solve = solve(&mut que_a, &mut vals, &mut resolved_eqs, &mut eq, root_a);
    let b_solve = solve(&mut que_b, &mut vals, &mut resolved_eqs, &mut eq, root_b);

    println!("{root_a} {a_solve:?}");
    println!("{root_b} {b_solve:?}");

    // println!("{vals:#?}", );

    todo!();
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
    const INPUT: &str = include_str!("../input/2022/day21.txt");

    #[test]
    fn day21_part2_test() {
        assert_eq!(part2_solve(EX), 301);
    }

    #[test]
    fn day21_part2_test_real() {
        assert_eq!(part2_solve(INPUT), 301);
    }
}
