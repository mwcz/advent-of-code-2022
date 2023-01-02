use aoc_runner_derive::aoc;
use std::collections::HashMap;

fn wrap(i: i32, len: i32) -> i32 {
    if i < 0 {
        ((len + i) % len) - 1
    } else if i > len {
        (i + 1) % len
    } else {
        i
    }
}

fn print(nums: &Vec<i32>, idx: &HashMap<i32, i32>) {
    // println!("{:?}", reconstruct(nums, idx));
}

fn reconstruct(nums: &Vec<i32>, idx: &HashMap<i32, i32>) -> Vec<i32> {
    let mut sorted_idx_map: Vec<(i32, i32)> = idx.iter().map(|(key, val)| (*key, *val)).collect();

    sorted_idx_map.sort_by(|a, b| a.1.cmp(&b.1));

    sorted_idx_map
        .iter()
        .map(|(key, _val)| nums[*key as usize])
        .collect()
}

/// Mix the i'th number from the original number list.
fn mix(nums: &Vec<i32>, idx: &mut HashMap<i32, i32>, i: i32) {
    let n = nums[i as usize];

    // get its current index
    let cur = *(idx.get(&i).unwrap());
    let new = wrap(n + cur, nums.len() as i32);

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

fn part1_solve(input: &str) -> i32 {
    // all ze numbaz
    let nums: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    // maps an original index to the mixed index
    let mut idx: HashMap<i32, i32> = nums
        .iter()
        .enumerate()
        .map(|(i, _)| (i as i32, i as i32))
        .collect();

    // println!("{:?}", nums);

    for (i, n) in nums.iter().enumerate() {
        // println!("\nmix position {i} ({n})",);
        mix(&nums, &mut idx, i as i32);
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
fn part1_solver(input: &str) -> i32 {
    part1_solve(input)
}

// #[aoc(day20, part2)]
// fn part2_solve(input: &str) -> i32 {
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
