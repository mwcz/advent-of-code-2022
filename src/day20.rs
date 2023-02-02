use aoc_runner_derive::aoc;
use std::collections::VecDeque;

// fn print(bowl: &VecDeque<(usize, i64)>) -> String {
//     format!("{:?}", bowl.iter().map(|n| n.1).collect::<Vec<i64>>())
// }

fn part1_solve(input: &str) -> i64 {
    // all ze numbaz
    let nums: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    // in a 4 quart mixing bowl...
    let mut bowl: VecDeque<(usize, i64)> = nums.clone().into_iter().enumerate().collect();

    for (i, num) in nums.into_iter().enumerate() {
        let num_pos = bowl.iter().position(|&n| n == (i, num)).unwrap();

        if num > 0 {
            // rotate the given number to the beginning of the ... bowl (this metaphor is already
            // breaking down)
            bowl.rotate_left(num_pos);
            bowl.pop_front();
            bowl.rotate_left((num as usize) % bowl.len());
            bowl.push_front((i, num));
        } else if num < 0 {
            // for negatives, rotate the number to the end of the bowl and then rotate the bowl to
            // the right
            bowl.rotate_right(bowl.len() - num_pos - 1);
            bowl.pop_back();
            let num_us = (num).abs() as usize;
            bowl.rotate_right(num_us % bowl.len());
            bowl.push_back((i, num));
        }
    }

    let zero = bowl.iter().position(|&n| n.1 == 0).unwrap();

    bowl.rotate_left(zero);

    println!("{:?}", bowl.iter().map(|n| n.1).collect::<Vec<i64>>());

    let mut spinning_bowl = bowl.into_iter().map(|n| n.1).cycle();

    let x = spinning_bowl.nth(1000).unwrap();
    let y = spinning_bowl.nth(999).unwrap();
    let z = spinning_bowl.nth(999).unwrap();

    println!("{:?}", (x, y, z));

    x + y + z
}

fn part2_solve(input: &str) -> i64 {
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

            if *num > 0 {
                // rotate the given number to the beginning of the ... bowl (this metaphor is already
                // breaking down)
                bowl.rotate_left(num_pos);
                bowl.pop_front();
                bowl.rotate_left((*num as usize) % bowl.len());
                bowl.push_front((i, *num));
            } else if *num < 0 {
                // for negatives, rotate the number to the end of the bowl and then rotate the bowl to
                // the right
                bowl.rotate_right(bowl.len() - num_pos - 1);
                bowl.pop_back();
                let num_us = (num).unsigned_abs() as usize;
                bowl.rotate_right(num_us % bowl.len());
                bowl.push_back((i, *num));
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

#[aoc(day20, part1)]
fn part1_solver(input: &str) -> i64 {
    part1_solve(input)
}

#[aoc(day20, part2)]
fn part2_solver(input: &str) -> i64 {
    part2_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn day20_part1_test() {
        assert_eq!(part1_solve(EX), 3);
    }

    #[test]
    fn day20_part2_test() {
        assert_eq!(part2_solve(EX), 1623178306);
    }
}

/* Day 20 original try using a vec, before switching to a vecdeque.

use aoc_runner_derive::aoc;
use std::collections::HashMap;

fn wrap(i: i64, len: i64) -> i64 {
    if i < 0 {
        ((len + i) % len) - 1
    } else if i > len {
        (i + 1) % len
    } else {
        i
    }
}

fn print(nums: &Vec<i64>, idx: &HashMap<i64, i64>) {
    // println!("{:?}", reconstruct(nums, idx));
}

fn reconstruct(nums: &Vec<i64>, idx: &HashMap<i64, i64>) -> Vec<i64> {
    let mut sorted_idx_map: Vec<(i64, i64)> = idx.iter().map(|(key, val)| (*key, *val)).collect();

    sorted_idx_map.sort_by(|a, b| a.1.cmp(&b.1));

    sorted_idx_map
        .iter()
        .map(|(key, _val)| nums[*key as usize])
        .collect()
}

/// Mix the i'th number from the original number list.
fn mix(nums: &Vec<i64>, idx: &mut HashMap<i64, i64>, i: i64) {
    let n = nums[i as usize];

    // get its current index
    let cur = *(idx.get(&i).unwrap());
    let new = wrap(n + cur, nums.len() as i64);

    if new == cur {
        print(nums, idx);
        return;
    }

    let (range, shift) = if new > cur {
        ((cur + 1)..(new + 1), -1)
    } else {
        (new..cur, 1)
    };

    for (_key, val) in idx.iter_mut() {
        if range.contains(val) {
            *val += shift;
        }
    }

    idx.insert(i, new);

    print(nums, idx);
}

fn part1_solve(input: &str) -> i64 {
    // all ze numbaz
    let nums: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    // maps an original index to the mixed index
    let mut idx: HashMap<i64, i64> = nums
        .iter()
        .enumerate()
        .map(|(i, _)| (i as i64, i as i64))
        .collect();

    // println!("{:?}", nums);

    for (i, n) in nums.iter().enumerate() {
        // println!("\nmix position {i} ({n})",);
        mix(&nums, &mut idx, i as i64);
    }

    let mixed = reconstruct(&nums, &idx);
    let zeropos = mixed.iter().position(|&n| n == 0).unwrap();
    // let mut mixed = mixed.iter().cycle();

    // mixed.find(|n| n == &&0);

    // let x = mixed.nth(999).unwrap();
    // let y = mixed.nth(999).unwrap();
    // let z = mixed.nth(999).unwrap();

    let x = mixed[(zeropos + 1000) % mixed.len()];
    let y = mixed[(zeropos + 2000) % mixed.len()];
    let z = mixed[(zeropos + 3000) % mixed.len()];

    println!("{x} {y} {z}");

    x + y + z
}

#[aoc(day20, part1)]
fn part1_solver(input: &str) -> i64 {
    part1_solve(input)
}

// #[aoc(day20, part2)]
// fn part2_solve(input: &str) -> i64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1
2
-3
3
-2
0
4";

    const EX2: &str = include_str!("../input/2022/day20.txt");

    #[test]
    fn day20_part1_test() {
        assert_eq!(part1_solve(EX), 3);
    }

    // #[test]
    // fn day20_part2_test() {
    //     assert_eq!(part2_solve(EX), 56);
    // }
}
*/
