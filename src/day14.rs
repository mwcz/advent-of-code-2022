use aoc_runner_derive::aoc;

#[aoc(day14, part1)]
fn part1_solve(input: &str) -> usize {
    const START: (u16, u16) = (500, 0);

    let mut lowest_rock: u16 = 0;

    let rock_vertices = input.lines().map(|line| {
        line.split(" -> ")
            .map(|pair| {
                let mut coords = pair.split(',').map(|n| n.parse::<u16>().unwrap());
                (coords.next().unwrap(), coords.next().unwrap())
            })
            .collect::<Vec<(u16, u16)>>()
    });

    let mut rocks: Vec<(u16, u16)> = vec![];

    for vertex in rock_vertices {
        for pair in vertex.windows(2) {
            let mut xrange = pair[0].0..=pair[1].0;
            if xrange.is_empty() {
                xrange = pair[1].0..=pair[0].0;
            }
            for x in xrange {
                let mut yrange = pair[0].1..=pair[1].1;
                if yrange.is_empty() {
                    yrange = pair[1].1..=pair[0].1;
                }
                for y in yrange {
                    rocks.push((x, y));

                    lowest_rock = lowest_rock.max(y);
                }
            }
        }
    }

    // a falling grain of sand
    let mut grain = START;

    // all the grains of settled sand
    let mut sand: Vec<(u16, u16)> = vec![];

    // process grains until one falls below the lowest rock
    while grain.1 < lowest_rock {
        // try to move grain by (0,1) or (-1,1) or (1,1), whichever is open first
        // if all three are blocked, leave grain where it is and push it into settled_sand then
        // clone a new active_grian from start

        let possible_dests = [
            (grain.0, grain.1 + 1),
            (grain.0 - 1, grain.1 + 1),
            (grain.0 + 1, grain.1 + 1),
        ];

        let dest = possible_dests
            .iter()
            .find(|coord| !rocks.contains(coord) && !sand.contains(coord));

        // move grain to its destination, or mark it as settled
        match dest {
            Some(coord) => grain = *coord,
            None => {
                sand.push(grain);
                grain = START;
            }
        }
    }

    sand.len()
}

#[aoc(day14, part2)]
fn part2_solve(input: &str) -> usize {
    const START: (u16, u16) = (500, 0);

    let mut lowest_rock: u16 = 0;

    let rock_vertices = input.lines().map(|line| {
        line.split(" -> ")
            .map(|pair| {
                let mut coords = pair.split(',').map(|n| n.parse::<u16>().unwrap());
                (coords.next().unwrap(), coords.next().unwrap())
            })
            .collect::<Vec<(u16, u16)>>()
    });

    let mut rocks: Vec<(u16, u16)> = vec![];

    for vertex in rock_vertices {
        for pair in vertex.windows(2) {
            let mut xrange = pair[0].0..=pair[1].0;
            if xrange.is_empty() {
                xrange = pair[1].0..=pair[0].0;
            }
            for x in xrange {
                let mut yrange = pair[0].1..=pair[1].1;
                if yrange.is_empty() {
                    yrange = pair[1].1..=pair[0].1;
                }
                for y in yrange {
                    rocks.push((x, y));

                    lowest_rock = lowest_rock.max(y);
                }
            }
        }
    }

    let floor = lowest_rock + 2;

    // a falling grain of sand
    let mut grain = START;

    // all the grains of settled sand
    let mut sand: Vec<(u16, u16)> = vec![];

    // process grains until one falls below the lowest rock
    loop {
        // try to move grain by (0,1) or (-1,1) or (1,1), whichever is open first
        // if all three are blocked, leave grain where it is and push it into settled_sand then
        // clone a new active_grian from start

        let possible_dests = [
            (grain.0, grain.1 + 1),
            (grain.0 - 1, grain.1 + 1),
            (grain.0 + 1, grain.1 + 1),
        ];

        let dest = possible_dests
            .iter()
            .find(|coord| !rocks.contains(coord) && !sand.contains(coord) && coord.1 < floor);

        // move grain to its destination, or mark it as settled
        match dest {
            Some(coord) => grain = *coord,
            None => {
                sand.push(grain);
                if grain == START {
                    break;
                }
                grain = START;
            }
        }
    }

    sand.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solve(EX), 24);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solve(EX), 93);
    }
}
