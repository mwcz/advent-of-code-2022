use std::collections::HashSet;
use std::iter::Filter;
use std::slice::Iter;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use itertools::Permutations;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cube {
    x: u16,
    y: u16,
    z: u16,
}
impl Cube {
    fn get_axis(&self, axis: char) -> u16 {
        match axis {
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!(),
        }
    }
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let mut axes = value.split(',');
        Self {
            x: axes.next().unwrap().parse().unwrap(),
            y: axes.next().unwrap().parse().unwrap(),
            z: axes.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Cubes {
    cubes: Vec<Cube>,
}

impl Cubes {
    fn new(input: &str) -> Self {
        let mut cubes = vec![];

        for line in input.lines() {
            let cube = Cube::from(line);
            cubes.push(cube);
        }

        Self {
            cubes,
        }
    }

    // /// Get the cubes along a single line.
    // fn axis(
    //     &self,
    //     axis1: char,
    //     rank1: u16,
    //     axis2: char,
    //     rank2: u16,
    // ) -> impl Iterator<Item = &Cube> {
    //     let seen1 = match axis1 {
    //         'x' => &self.seen_x,
    //         'y' => &self.seen_y,
    //         'z' => &self.seen_z,
    //         _ => panic!(),
    //     };

    //     let seen2 = match axis2 {
    //         'x' => &self.seen_x,
    //         'y' => &self.seen_y,
    //         'z' => &self.seen_z,
    //         _ => panic!(),
    //     };

    //     self.cubes
    //         .iter()
    //         .filter(move |cube| cube.get_axis(axis1) == rank1 && cube.get_axis(axis2) == rank2)
    // }

    // fn adjacent<'a>(cubes: &'a Vec<&Cube>, check_axis: char) -> Vec<&'a Cube> {
    //     let mut adj = vec![];
    //     // create a map of all adjacent cubes.  a pair of cubes are adjacent if they have two axes
    //     // in common, and the remaining axis is +/- 1.  ex: 2,2,1 and 2,2,2.
    //     // get all the pairs of cubes along this axis/rank
    //     for pair in cubes.iter().combinations(2) {
    //         let a = pair[0];
    //         let b = pair[1];
    //         let dist = match check_axis {
    //             'x' => a.x.abs_diff(b.x),
    //             'y' => a.y.abs_diff(b.y),
    //             'z' => a.z.abs_diff(b.z),
    //             _ => panic!(),
    //         };
    //         if dist == 1 {
    //             println!("ADJACENT: {:?} {:?}", a, b);
    //             adj.push(*a);
    //             adj.push(*b);
    //         }
    //     }
    //     adj
    // }

    // fn faces_on_axis(&self, axis1: char, rank1: u16, axis2: char, rank2: u16) -> usize {
    //     let cubes_on_axis = self.axis(axis1, rank1, axis2, rank2).collect_vec();
    //     let adjacent_cubes = Cubes::adjacent(
    //         &cubes_on_axis,
    //         *(['x', 'y', 'z']
    //             .iter()
    //             .filter(|&a| a != &axis1 && a != &axis2)
    //             .next()
    //             .unwrap()),
    //     );

    //     println!("cubes: {} ({} adjacent)", cubes_on_axis.len(), adjacent_cubes.len());

    //     2 * cubes_on_axis.len() - adjacent_cubes.len()
    // }
}

#[aoc(day18, part1)]
fn part1_solve(input: &str) -> usize {
    let cubes = input.lines().map(Cube::from).collect_vec();

    let mut faces = cubes.len() * 6;

    for pair in cubes.iter().combinations(2) {
        let a = pair[0];
        let b = pair[1];

        let xd = a.x.abs_diff(b.x) ;
        let yd = a.y.abs_diff(b.y) ;
        let zd = a.z.abs_diff(b.z) ;

        if xd + yd + zd == 1 {
            faces -= 2;
        }
    }

    faces
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solve(EX), 64);
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(part2_solve(EX), 1514285714288);
    // }
}
