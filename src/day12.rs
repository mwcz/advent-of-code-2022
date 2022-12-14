use aoc_runner_derive::aoc;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

/// Unique height value for start that makes all adjacent heights reachable.
const START_HEIGHT: u16 = 1;
/// Unique height value for end node that is reachable from any adjacent height.
const END_HEIGHT: u16 = 26;

#[aoc(day12, part1)]
fn part1_solve(input: &str) -> i32 {
    // let mut g = Graph::new();
    // let a = g.add_node( (0, 0) );
    // let b = g.add_node( (0, 0) );

    let mut graph = Graph::new();
    let mut start_idx = (0, 0);
    let mut end_idx = (0, 0);

    let grid: Vec<Vec<NodeIndex>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = match c {
                        'S' => {
                            start_idx = (y, x);
                            START_HEIGHT
                        }
                        'E' => {
                            end_idx = (y, x);
                            END_HEIGHT
                        }
                        _ => (c as u16) - 96,
                    };
                    graph.add_node((y, x, height))
                })
                .collect()
        })
        .collect();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let node = graph[grid[y][x]];

            let above = (y.checked_add(1), Some(x));
            let below = (y.checked_sub(1), Some(x));
            let left = (Some(y), x.checked_sub(1));
            let right = (Some(y), x.checked_add(1));

            for dir in [above, below, left, right] {
                let (Some(adj_y), Some(adj_x)) = dir else{
                    continue;
                };
                // if adj_y and adj_x are in range
                if (0..grid.len()).contains(&adj_y) && (0..grid[0].len()).contains(&adj_x) {
                    let adj_node = graph[grid[adj_y][adj_x]];
                    if Some(adj_node.2) <= node.2.checked_add(1) {
                        graph.add_edge(grid[y][x], grid[adj_y][adj_x], 1);
                    }
                }
            }
        }
    }

    let start = grid[start_idx.0][start_idx.1];
    let end = grid[end_idx.0][end_idx.1];

    let path = astar(&graph, start, |finish| finish == end, |_| 1, |_| 1);

    path.unwrap().0
}

#[aoc(day12, part2)]
fn part2_solve(input: &str) -> i32 {
    // let mut g = Graph::new();
    // let a = g.add_node( (0, 0) );
    // let b = g.add_node( (0, 0) );

    let mut graph = Graph::new();
    let mut start_idx = (0, 0);
    let mut end_idx = (0, 0);

    let grid: Vec<Vec<NodeIndex>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = match c {
                        'S' => {
                            start_idx = (y, x);
                            START_HEIGHT
                        }
                        'E' => {
                            end_idx = (y, x);
                            END_HEIGHT
                        }
                        _ => (c as u16) - 96,
                    };
                    graph.add_node((y, x, height))
                })
                .collect()
        })
        .collect();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let node = graph[grid[y][x]];

            let above = (y.checked_add(1), Some(x));
            let below = (y.checked_sub(1), Some(x));
            let left = (Some(y), x.checked_sub(1));
            let right = (Some(y), x.checked_add(1));

            for dir in [above, below, left, right] {
                let (Some(adj_y), Some(adj_x)) = dir else{
                    continue;
                };
                // if adj_y and adj_x are in range
                if (0..grid.len()).contains(&adj_y) && (0..grid[0].len()).contains(&adj_x) {
                    let adj_node = graph[grid[adj_y][adj_x]];
                    if Some(adj_node.2) <= node.2.checked_add(1) {
                        graph.add_edge(grid[y][x], grid[adj_y][adj_x], 1);
                    }
                }
            }
        }
    }

    let end = grid[end_idx.0][end_idx.1];

    let mut dists = Vec::new();

    for node_idx in graph.node_indices() {
        let node = graph[node_idx];
        if node.2 == START_HEIGHT {
            let path = astar(&graph, node_idx, |finish| finish == end, |_| 1, |_| 1);

            if let Some(path) = path {
                dists.push(path.0);
            }
        }
    }

    *dists.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Sabqponm\n\
                      abcryxxl\n\
                      accszExk\n\
                      acctuvwj\n\
                      abdefghi";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solve(EX), 31);
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(part2_solve(EX), 2713310158u64);
    // }
}
