mod parse;

use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, space1, u32 as nom_u32},
    combinator::map,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Log<'name> {
    Dir { name: &'name str },
    File { name: &'name str, size: u32 },
    Ls,
    Cd { name: &'name str },
}

#[derive(Debug, PartialEq)]
struct Inode<'name> {
    name: &'name str,
    /// The size of the inode.  0 for dirs, >0 for files.
    size: u32,
    /// The size of all the contents of this inode (for dirs).
    /// The index of the parent inode.  Only the root node has None.
    parent: Option<usize>,
    /// The index of this inode in the inode table.
    idx: usize,
}

#[derive(Debug)]
struct Filesystem<'inode> {
    inodes: Vec<Inode<'inode>>,
}

impl<'inode> Filesystem<'inode> {
    fn new(logs: Vec<Log<'inode>>) -> Filesystem<'inode> {
        let mut cwd: usize = 0;

        let mut fs = Filesystem { inodes: vec![] };

        for log in logs {
            match log {
                Log::Cd { name } => {
                    if name == ".." {
                        if let Some(parent) = fs.get_inode(Some(cwd)).unwrap().parent {
                            cwd = parent;
                        }
                    } else {
                        match fs.get_inode_in(name, cwd) {
                            Some(inode) => cwd = inode.idx,
                            None => cwd = fs.add_inode(name, 0, Some(cwd)),
                        }
                    }
                }
                Log::Ls => {}
                Log::Dir { name } => {
                    fs.add_inode(name, 0, Some(cwd));
                }
                Log::File { name, size } => {
                    fs.add_inode(name, size, Some(cwd));
                }
            }
        }

        fs
    }

    /// Add an inode.  Returns the index.
    fn add_inode(&mut self, name: &'inode str, size: u32, parent: Option<usize>) -> usize {
        let idx = self.inodes.len();
        // mark the root inode as having no parent
        let parent = if name == "/" { None } else { parent };
        self.inodes.push(Inode {
            name,
            size,
            parent,
            idx,
        });
        idx
    }

    fn get_inode(&self, idx: Option<usize>) -> Option<&Inode<'inode>> {
        self.inodes.get(idx.unwrap_or(0))
    }

    fn get_inode_in(&self, name: &str, in_dir: usize) -> Option<&Inode<'inode>> {
        self.inodes
            .iter()
            .find(|inode| inode.parent == Some(in_dir) && inode.name == name)
    }

    fn dir_size(&self, idx: Option<usize>) -> u32 {
        let (dirs, files): (Vec<_>, Vec<_>) = self
            .inodes
            .iter()
            .filter(|inode| inode.parent == idx)
            .partition(|inode| inode.size == 0);

        let files_size = files.iter().map(|file| file.size).sum::<u32>();

        let dirs_size = dirs
            .iter()
            .map(|dir| self.dir_size(Some(dir.idx)))
            .sum::<u32>();

        files_size + dirs_size
    }

    fn sum_under(&self, max: u32) -> u32 {
        self.inodes
            .iter()
            .filter_map(|inode| {
                if inode.size == 0 {
                    let dir_size = self.dir_size(Some(inode.idx));
                    if dir_size <= max {
                        return Some(dir_size);
                    }
                }
                None
            })
            .sum()
    }

    fn used_space(&self) -> u32 {
        self.dir_size(
            self.inodes
                .iter()
                .find(|&inode| inode.name == "/")
                .map(|inode| inode.idx),
        )
    }

    fn free_up(&self, total: u32, needed: u32) -> Option<u32> {
        let used = self.used_space();

        self.inodes
            .iter()
            .filter_map(|inode| {
                if inode.size == 0 {
                    let dir_size = self.dir_size(Some(inode.idx));
                    if total - (used - dir_size) > needed {
                        return Some(dir_size);
                    }
                }
                None
            })
            .min()
    }
}

fn part1_solve(input: &str) -> u32 {
    let (_, entries) = parse::log(input).expect("could not parse input");

    let fs = Filesystem::new(entries);

    fs.sum_under(100000)
}

#[aoc(day7, part1)]
fn part1_solver(input: &str) -> u32 {
    part1_solve(input)
}

fn part2_solve(input: &str) -> u32 {
    let (_, entries) = parse::log(input).expect("could not parse input");

    let fs = Filesystem::new(entries);

    fs.free_up(70_000_000, 30_000_000)
        .expect("no dir found that can free up enough space")
}

#[aoc(day7, part2)]
fn part2_solver(input: &str) -> u32 {
    part2_solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    const REAL: &str = include_str!("../input/2022/day7.txt");

    #[test]
    fn day7_part1_example() {
        assert_eq!(part1_solve(EX), 95437);
    }

    #[test]
    fn day7_part1_real() {
        assert_eq!(part1_solve(REAL), 1232307);
    }

    #[test]
    fn day7_part2_example() {
        assert_eq!(part2_solve(EX), 24933642);
    }

    #[test]
    fn day7_part2_real() {
        assert_eq!(part2_solve(REAL), 7268994);
    }
}
