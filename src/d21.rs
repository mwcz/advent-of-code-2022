use std::collections::{HashMap, VecDeque};

type Parsed = String;

pub fn parse(input: String) -> Parsed {
    input
}

pub fn part1(input: Parsed) -> i128 {
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

        if let Value::Pending((name, (a, op, b))) = next {
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
        } else {
            // not pending: remove it from the queue
            q.pop_back();
        }
    }

    // println!("{:#?}", vals);

    let Value::Resolved(ans) = vals.get("root").unwrap() else {
        panic!("root still unresolved!");
    };

    ans.1
}

pub fn part2(input: Parsed) -> String {
    let mut vals: HashMap<Name, Value> = HashMap::new();
    // let mut resolved_eqs: HashSet<Name> = HashSet::new();
    // The parts of the final equation
    // let mut eq: Vec<Name> = Vec::new();
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
        } else if strs.len() == 2 {
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

    fn solve<'a>(
        q: &mut VecDeque<&'a str>,
        vals: &mut HashMap<Name<'a>, Value<'a>>,
        start: &str,
    ) -> Option<i128> {
        use Value::*;
        while !q.is_empty() {
            let cur_name = *q.back().unwrap();
            let cur = vals.get(cur_name).unwrap();

            match cur {
                Pending((name, (a_name, op, b_name))) => {
                    let a = vals.get(a_name).unwrap();
                    let b = vals.get(b_name).unwrap();

                    if a.is_pending() {
                        q.push_back(a.name());
                    }

                    if b.is_pending() {
                        q.push_back(b.name());
                    }

                    // if both sub-expressions are resolved, compute them and resolve this expression
                    match (a, b) {
                        (Resolved((_, aval)), Resolved((_, bval))) => {
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
                            // q.pop_back();
                        }
                        (
                            Resolved(_) | Symbol(_) | Blocked(_),
                            Resolved(_) | Symbol(_) | Blocked(_),
                        ) => {
                            // if the parts are a combination of blocked, resolved, and symbol,
                            // there's nothing more to process for now, so mark this one as
                            // blocked.
                            vals.insert(name, Value::Blocked((name, (a_name, *op, b_name))));
                        }
                        _ => {}
                    }
                }
                Blocked((_name, (_a_name, _op, _b_name))) => {
                    q.pop_back();
                }
                _ => {
                    // not pending: remove it from the queue
                    q.pop_back();
                }
            }
        }
        vals.get(start).unwrap().val()
    }

    let root: Value = *vals.get("root").unwrap();

    // first operand of root's formula
    let Value::Pending((_, (a, _op, b))) = root else {
        panic!("root must start pending");
    };

    let mut que_a = VecDeque::from([a]);
    let mut que_b = VecDeque::from([b]);

    solve(&mut que_a, &mut vals, a);
    solve(&mut que_b, &mut vals, b);

    // println!("{a} {a_solve:?}");
    // println!("{b} {b_solve:?}");

    // println!("{vals:#?}", );

    let a_val = vals.get(a).unwrap();
    let b_val = vals.get(b).unwrap();

    format!("{} = {}", a_val.stringify(&vals), b_val.stringify(&vals))
}

type Name<'a> = &'a str;
type Operator = char;
#[derive(Debug, Clone, Copy)]
enum Value<'a> {
    Symbol(Name<'a>),
    Resolved((Name<'a>, i128)),
    Pending((Name<'a>, Formula<'a>)),
    Blocked((Name<'a>, Formula<'a>)),
}
type Formula<'a> = (Name<'a>, Operator, Name<'a>);

impl<'a> Value<'a> {
    fn name(&self) -> Name<'a> {
        match self {
            Value::Resolved(a) => a.0,
            Value::Pending(a) => a.0,
            Value::Symbol(a) => a,
            Value::Blocked(a) => a.0,
        }
    }
    fn val(&self) -> Option<i128> {
        if let Value::Resolved((_, val)) = self {
            Some(*val)
        } else {
            None
        }
    }
    fn is_pending(&self) -> bool {
        matches!(self, Value::Pending(_))
    }
    fn stringify(&self, vals: &HashMap<Name, Value>) -> String {
        match self {
            Value::Symbol(name) => name.to_string(),
            Value::Resolved((_, val)) => format!("{val}"),
            Value::Pending((name, (a, op, b))) => format!("{name} = {a} {op} {b}"),
            Value::Blocked((_, (a, op, b))) => format!(
                "({}) {op} ({})",
                vals.get(a).unwrap().stringify(vals),
                vals.get(b).unwrap().stringify(vals)
            ),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const EX: &str = "root: pppw + sjmn
// dbpl: 5
// cczh: sllz + lgvd
// zczc: 2
// ptdq: humn - dvpt
// dvpt: 3
// lfqf: 4
// humn: 5
// ljgn: 2
// sjmn: drzm * dbpl
// sllz: 4
// pppw: cczh / lfqf
// lgvd: ljgn * ptdq
// drzm: hmdt - zczc
// hmdt: 32";
//     const INPUT: &str = include_str!("../input/2022/day21.txt");
//
//     #[test]
//     fn day21_part1_test() {
//         assert_eq!(part1_solve(EX), 152);
//     }
//
//     #[test]
//     fn day21_part1_real() {
//         assert_eq!(part1_solve(INPUT), 194058098264286);
//     }
//
//     // #[test]
//     // fn day21_part2_test() {
//     //     assert_eq!(part2_solve(EX), 301);
//     // }
//
//     // #[test]
//     // fn day21_part2_test_real() {
//     //     assert_eq!(part2_solve(INPUT), 301);
//     // }
// }
