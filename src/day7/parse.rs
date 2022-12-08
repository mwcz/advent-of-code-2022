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

