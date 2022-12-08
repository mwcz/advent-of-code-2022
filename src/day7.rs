use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, space1, u32 as nom_u32},
    combinator::map,
    sequence::tuple,
    IResult,
};
use patricia_tree::PatriciaMap;

#[derive(Debug, PartialEq)]
pub enum Entry<'name> {
    Dir { name: &'name str },
    File { name: &'name str, size: u32 },
    Ls,
    Cd { name: &'name str },
}

mod parse {
    use nom::{branch::alt, multi::separated_list0};

    use super::*;

    pub fn log(input: &str) -> IResult<&str, Vec<Entry>> {
        separated_list0(newline, alt((ls, cd, file, dir)))(input)
    }

    fn ls(input: &str) -> IResult<&str, Entry> {
        map(tag("$ ls"), |_| Entry::Ls)(input)
    }

    fn cd(input: &str) -> IResult<&str, Entry> {
        map(tuple((tag("$ cd "), not_line_ending)), |tup| Entry::Cd {
            name: tup.1,
        })(input)
    }

    fn file(input: &str) -> IResult<&str, Entry> {
        map(tuple((nom_u32, space1, not_line_ending)), |tup| {
            Entry::File {
                name: tup.2,
                size: tup.0,
            }
        })(input)
    }

    fn dir(input: &str) -> IResult<&str, Entry> {
        map(tuple((tag("dir "), not_line_ending)), |tup| Entry::Dir {
            name: tup.1,
        })(input)
    }

    #[test]
    fn parse_test() {
        assert_eq!(ls("$ ls"), Ok(("", Entry::Ls)));
        assert_eq!(cd("$ cd .."), Ok(("", Entry::Cd { name: ".." })));
        #[rustfmt::skip]
        assert_eq!(file("1234 foo.txt"), Ok(("", Entry::File { name: "foo.txt", size: 1234 })));
        assert_eq!(dir("dir bar"), Ok(("", Entry::Dir { name: "bar" })));
        assert_eq!(
            log("$ ls\n\
                 $ cd ..\n\
                 1234 foo.txt\n\
                 dir bar"),
            Ok((
                "",
                vec![
                    Entry::Ls,
                    Entry::Cd { name: ".." },
                    Entry::File {
                        name: "foo.txt",
                        size: 1234
                    },
                    Entry::Dir { name: "bar" }
                ]
            ))
        );
    }
}

#[derive(Debug, PartialEq)]
enum Inode<'name> {
    Dir {
        name: &'name str,
        contents: Vec<Inode<'name>>,
    },
    File {
        name: &'name str,
        size: usize,
    },
}

#[derive(Debug)]
struct Device<'inode> {
    cwd: Vec<&'inode str>,
    tree: PatriciaMap<Inode<'inode>>,
}

impl<'inode> Device<'inode> {
    fn new() -> Device<'inode> {
        Device {
            cwd: vec![],
            tree: PatriciaMap::new(),
        }
    }

    fn sum_under(&mut self, entries: Vec<Entry<'inode>>, max: u32) -> u32 {
        let mut sum = 0;
        let mut dir_sum = 0;
        for entry in entries {
            match entry {
                Entry::Cd { name } => {
                    // if name == ".." {
                    //     self.cwd.pop();
                    // } else {
                    //     self.cwd.push(name);
                    // }
                }
                Entry::Ls => {}
                Entry::Dir { name } => {
                    if dir_sum <= max {
                        sum += dir_sum;
                    }
                    dir_sum = 0;
                    // self.tree.insert(name, Inode::Dir { name });
                }
                Entry::File { name, size } => {
                    dir_sum += size;
                }
            }
        }

        sum
    }
}

#[test]
fn day7_test() {
    let ex = Device::new().sum_under(
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
        .unwrap().1,
        100000,
    );
    assert_eq!(ex, 95437);
}

#[aoc(day7, part1)]
fn part1_solve(input: &str) -> u32 {
    let (_, entries) = parse::log(input).expect("could not parse input");

    let mut device = Device::new();

    device.sum_under(entries, 100000)
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
