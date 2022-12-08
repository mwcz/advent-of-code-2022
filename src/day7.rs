mod parse;

use std::{cell::RefCell, rc::Rc};

use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, space1, u32 as nom_u32},
    combinator::map,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Entry<'name> {
    Dir { name: &'name str },
    File { name: &'name str, size: u32 },
    Ls,
    Cd { name: &'name str },
}

type RcDir<'name> = Rc<RefCell<Dir<'name>>>;
type RcFile<'name> = Rc<RefCell<File<'name>>>;

#[derive(Debug, PartialEq)]
struct Dir<'name> {
    name: &'name str,
    dirs: Vec<RcDir<'name>>,
    files: Vec<RcFile<'name>>,
    parent: Option<RcDir<'name>>,
}

impl<'name> Dir<'name> {
    /// Get a direct child dir by name.
    fn get_dir(&self, name: &'name str) -> Option<&'name RcDir> {
        self.dirs.iter().find(|&dir| dir.borrow().name == name)
    }
    fn add_dir(&mut self, parent: RcDir<'name>, name: &'name str) {
        self.dirs.push(Rc::new(RefCell::new(Dir {
            name,
            dirs: vec![],
            files: vec![],
            parent: Some(parent),
        })));
    }
    fn add_file(&mut self, name: &'name str, size: u32) {
        self.files.push(Rc::new(RefCell::new(File { name, size })));
    }
}

#[derive(Debug, PartialEq)]
struct File<'name> {
    name: &'name str,
    size: u32,
}

#[derive(Debug)]
struct Filesystem<'inode> {
    cwd: Vec<&'inode str>,
    root: RcDir<'inode>,
}

impl<'inode> Filesystem<'inode> {
    fn new(entries: Vec<Entry<'inode>>) -> Filesystem<'inode> {
        let root = Rc::new(RefCell::new(Dir {
            name: "/",
            dirs: vec![],
            files: vec![],
            parent: None,
        }));
        let mut cwd = root.clone();

        for entry in entries {
            match entry {
                Entry::Cd { name } => {
                    if name == ".." {
                        cwd = cwd
                            .borrow()
                            .parent
                            .expect("can't move '..' from the root directory");
                    } else {
                        cwd = *cwd
                            .borrow()
                            .get_dir(name)
                            .expect("tried to cd into a nonexistant dir");
                    }
                }
                Entry::Ls => {}
                Entry::Dir { name } => {
                    cwd.borrow().add_dir(cwd, name);
                }
                Entry::File { name, size } => {
                    cwd.borrow().add_file(name, size);
                }
            }
        }

        Filesystem { cwd: vec![], root }
    }

    /// Get the size of a file or total size of a directory.
    fn stat(&self, path: Vec<&str>) -> Option<u32> {
        let mut inode = &self.root;

        for seg in path {
            // if
        }
        todo!();
    }

    fn sum_under(&mut self, max: u32) -> u32 {
        let mut sum = 0;
        let mut dir_sum = 0;

        sum
    }
}

#[test]
fn day7_test() {
    let ex = Filesystem::new(
        parse::log(
            "$ cd /
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
7214296 k",
        )
        .unwrap()
        .1,
    );
    // assert_eq!(ex, 95437);
}

#[aoc(day7, part1)]
fn part1_solve(input: &str) -> u32 {
    // let (_, entries) = parse::log(input).expect("could not parse input");

    // let mut device = Filesystem::new();

    // device.sum_under(entries, 100000)
    todo!();
}

// #[aoc(day7, part2)]
// fn part2_solve(input: &str) -> usize {
// }

// #[derive(Debug, PartialEq)]
// struct Device<'dir> {
//     cwd: Dir<'dir>,
// }

// #[derive(Debug, PartialEq)]
// struct File<'name> {
//     name: &'name str,
//     size: u32,
// }

// #[derive(Debug, PartialEq)]
// enum Command<'dir> {
//     Cd(DirMove<'dir>),
//     Ls,
// }

// impl<'com> TryFrom<&'com str> for Command<'com> {
//     type Error = ();

//     fn try_from(value: &'com str) -> Result<Self, Self::Error> {
//         let mut words = value.split_whitespace();

//         if let Some(command_name) = words.next() {
//             match command_name {
//                 "cd" => {
//                     if let Some(dir) = words.next() {
//                         Ok(Command::Cd(dir.into()))
//                     } else {
//                         Err(())
//                     }
//                 }
//                 "ls" => Ok(Command::Ls),
//                 _ => Err(()),
//             }
//         } else {
//             Err(())
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// enum DirMove<'path> {
//     Path(&'path str),
//     Up,
// }

// impl<'dir> From<&'dir str> for DirMove<'dir> {
//     fn from(value: &'dir str) -> Self {
//         match value {
//             ".." => DirMove::Up,
//             _ => DirMove::Path(value),
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// struct Dir<'name> {
//     name: &'name str,
//     files: Vec<File<'name>>,
// }

// #[cfg(test)]
// mod day7_tests {
//     use super::*;

//     #[test]
//     fn parse_test() {
//         assert_eq!(DirMove::from(".."), DirMove::Up);
//         assert_eq!(DirMove::from("foo"), DirMove::Path("foo"));
//         assert_eq!(Command::try_from("ls"), Ok(Command::Ls));
//         assert_eq!(Command::try_from("cd .."), Ok(Command::Cd(DirMove::Up)));
//         assert_eq!(
//             Command::try_from("cd foo"),
//             Ok(Command::Cd(DirMove::Path("foo")))
//         );
//     }
// }
