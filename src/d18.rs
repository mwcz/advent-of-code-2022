use itertools::Itertools;

type Parsed = String;

pub fn parse(input: String) -> Parsed {
    input
}

pub fn part1(input: Parsed) -> usize {
    let cubes = input.lines().map(Cube::from).collect_vec();

    let mut faces = cubes.len() * 6;

    for pair in cubes.iter().combinations(2) {
        let a = pair[0];
        let b = pair[1];

        let xd = a.x.abs_diff(b.x);
        let yd = a.y.abs_diff(b.y);
        let zd = a.z.abs_diff(b.z);

        if xd + yd + zd == 1 {
            faces -= 2;
        }
    }

    faces
}

pub fn part2(input: Parsed) -> usize {
    let cubes = input.lines().map(Cube::from).collect_vec();

    let mut x = (0, 0);
    let mut y = (0, 0);
    let mut z = (0, 0);
    // bounding box
    for cube in &cubes {
        x = (x.0.min(cube.x), x.1.max(cube.x));
        y = (y.0.min(cube.y), y.1.max(cube.y));
        z = (z.0.min(cube.z), z.1.max(cube.z));
    }

    // add an extra row of padding on either end of each axis to ensure the "steam" can flow around
    // structures touching the perimeter of the bounding box

    const PAD: usize = 3;
    let mut space = vec![vec![vec![false; z.1 - z.0 + PAD]; y.1 - y.0 + PAD]; x.1 - x.0 + PAD];

    // fill in the cubes
    for cube in &cubes {
        space[cube.x - x.0 + PAD / 2][cube.y - y.0 + PAD / 2][cube.z - z.0 + PAD / 2] = true;
    }

    fn steam(
        space: &Vec<Vec<Vec<bool>>>,
        visited: &mut Vec<Vec<Vec<bool>>>,
        (x, y, z): (usize, usize, usize),
        faces: &mut usize,
    ) {
        let mut directions = vec![];
        visited[x][y][z] = true;

        // add valid directions for steam to travel which don't hit the bounding box

        if x > 0 {
            directions.push((x - 1, y, z));
        }
        if x + 1 < space.len() {
            directions.push((x + 1, y, z));
        }
        if y > 0 {
            directions.push((x, y - 1, z));
        }
        if y + 1 < space[0].len() {
            directions.push((x, y + 1, z));
        }
        if z > 0 {
            directions.push((x, y, z - 1));
        }
        if z + 1 < space[0][0].len() {
            directions.push((x, y, z + 1));
        }

        let open_directions = directions
            .into_iter()
            .filter(|&(lx, ly, lz)| {
                let has_lava = space[lx][ly][lz];
                let already_visited = visited[lx][ly][lz];

                if has_lava {
                    *faces += 1;
                }

                let open = !has_lava && !already_visited;

                if open {
                    visited[lx][ly][lz] = true;
                    true
                } else {
                    false
                }
            })
            .collect_vec();

        // let open_faces = open_directions.len();
        // faces += 6 - open_faces;

        open_directions
            .iter()
            .for_each(|&dir| steam(space, visited, dir, faces));
    }

    // start at origin; no lava can be there because it's within the padding
    let mut visited = vec![vec![vec![false; z.1 - z.0 + PAD]; y.1 - y.0 + PAD]; x.1 - x.0 + PAD];
    let mut faces = 0;
    steam(&space, &mut visited, (0, 0, 0), &mut faces);

    faces
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const EX: &str = "2,2,2
// 1,2,2
// 3,2,2
// 2,1,2
// 2,3,2
// 2,2,1
// 2,2,3
// 2,2,4
// 2,2,6
// 1,2,5
// 3,2,5
// 2,1,5
// 2,3,5";
//
//     // const EX2: &str = "1,1,0
//     // 0,1,1
//     // 1,0,1
//     // 1,2,1
//     // 2,1,1
//     // 1,1,2";
//
//     #[test]
//     fn part1_test() {
//         assert_eq!(part1_solve(EX), 64);
//     }
//
//     #[test]
//     fn part2_test() {
//         assert_eq!(part2_solve(EX), 58);
//         // assert_eq!(part2_solve(EX2), 30);
//     }
// }
