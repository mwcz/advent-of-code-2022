use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::VecDeque,
};

type Parsed = String;

pub fn parse(input: String) -> Parsed {
    input
}

pub fn part1(input: Parsed) -> i64 {
    // all ze numbaz
    let nums: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    // in a 4 quart mixing bowl...
    let mut bowl: VecDeque<(usize, i64)> = nums.clone().into_iter().enumerate().collect();

    for (i, num) in nums.into_iter().enumerate() {
        let num_pos = bowl.iter().position(|&n| n == (i, num)).unwrap();

        match num.cmp(&0) {
            Less => {
                // for negatives, rotate the number to the end of the bowl and then rotate the bowl to
                // the right
                bowl.rotate_right(bowl.len() - num_pos - 1);
                bowl.pop_back();
                let num_us = (num).unsigned_abs() as usize;
                bowl.rotate_right(num_us % bowl.len());
                bowl.push_back((i, num));
            }
            Greater => {
                // rotate the given number to the beginning of the ... bowl (this metaphor is already
                // breaking down)
                bowl.rotate_left(num_pos);
                bowl.pop_front();
                bowl.rotate_left((num as usize) % bowl.len());
                bowl.push_front((i, num));
            }
            Equal => {}
        }
    }

    let zero = bowl.iter().position(|&n| n.1 == 0).unwrap();

    bowl.rotate_left(zero);

    // println!("{:?}", bowl.iter().map(|n| n.1).collect::<Vec<i64>>());

    let mut spinning_bowl = bowl.into_iter().map(|n| n.1).cycle();

    let x = spinning_bowl.nth(1000).unwrap();
    let y = spinning_bowl.nth(999).unwrap();
    let z = spinning_bowl.nth(999).unwrap();

    // println!("{:?}", (x, y, z));

    x + y + z
}

pub fn part2(input: Parsed) -> i64 {
    // all ze numbaz
    let nums: Vec<i64> = input
        .lines()
        .map(|line| 811589153 * line.parse::<i64>().unwrap())
        .collect();

    // in a 4 quart mixing bowl...
    let mut bowl: VecDeque<(usize, i64)> = nums.clone().into_iter().enumerate().collect();

    for _ in 0..10 {
        for (i, num) in nums.iter().enumerate() {
            let num_pos = bowl.iter().position(|&n| n == (i, *num)).unwrap();

            match num.cmp(&0) {
                Less => {
                    // for negatives, rotate the number to the end of the bowl and then rotate the bowl to
                    // the right
                    bowl.rotate_right(bowl.len() - num_pos - 1);
                    bowl.pop_back();
                    let num_us = (num).unsigned_abs() as usize;
                    bowl.rotate_right(num_us % bowl.len());
                    bowl.push_back((i, *num));
                }
                Greater => {
                    // rotate the given number to the beginning of the ... bowl (this metaphor is already
                    // breaking down)
                    bowl.rotate_left(num_pos);
                    bowl.pop_front();
                    bowl.rotate_left((*num as usize) % bowl.len());
                    bowl.push_front((i, *num));
                }
                Equal => {}
            }
        }
    }

    let zero = bowl.iter().position(|&n| n.1 == 0).unwrap();

    bowl.rotate_left(zero);

    // println!("{:?}", bowl.iter().map(|n| n.1).collect::<Vec<i64>>());

    let mut spinning_bowl = bowl.into_iter().map(|n| n.1).cycle();

    spinning_bowl.next(); // consume the 0

    let x = spinning_bowl.nth(999).unwrap();
    let y = spinning_bowl.nth(999).unwrap();
    let z = spinning_bowl.nth(999).unwrap();

    // println!("{:?}", (x,y,z));

    x + y + z
}

// fn print(bowl: &VecDeque<(usize, i64)>) -> String {
//     format!("{:?}", bowl.iter().map(|n| n.1).collect::<Vec<i64>>())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const EX: &str = "1
// 2
// -3
// 3
// -2
// 0
// 4";
//
//     #[test]
//     fn day20_part1_test() {
//         assert_eq!(part1_solve(EX), 3);
//     }
//
//     #[test]
//     fn day20_part2_test() {
//         assert_eq!(part2_solve(EX), 1623178306);
//     }
// }
