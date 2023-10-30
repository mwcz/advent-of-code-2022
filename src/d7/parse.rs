use nom::{branch::alt, multi::separated_list0};

use super::*;

pub fn log(input: &str) -> IResult<&str, Vec<Log>> {
    separated_list0(newline, alt((ls, cd, file, dir)))(input)
}

fn ls(input: &str) -> IResult<&str, Log> {
    map(tag("$ ls"), |_| Log::Ls)(input)
}

fn cd(input: &str) -> IResult<&str, Log> {
    map(tuple((tag("$ cd "), not_line_ending)), |tup| Log::Cd {
        name: tup.1,
    })(input)
}

fn file(input: &str) -> IResult<&str, Log> {
    map(tuple((nom_u32, space1, not_line_ending)), |tup| Log::File {
        name: tup.2,
        size: tup.0,
    })(input)
}

fn dir(input: &str) -> IResult<&str, Log> {
    map(tuple((tag("dir "), not_line_ending)), |tup| Log::Dir {
        name: tup.1,
    })(input)
}

#[test]
fn parse_test() {
    assert_eq!(ls("$ ls"), Ok(("", Log::Ls)));
    assert_eq!(cd("$ cd .."), Ok(("", Log::Cd { name: ".." })));
    #[rustfmt::skip]
    assert_eq!(file("1234 foo.txt"), Ok(("", Log::File { name: "foo.txt", size: 1234 })));
    assert_eq!(dir("dir bar"), Ok(("", Log::Dir { name: "bar" })));
    assert_eq!(
        log("$ ls\n\
                 $ cd ..\n\
                 1234 foo.txt\n\
                 dir bar"),
        Ok((
            "",
            vec![
                Log::Ls,
                Log::Cd { name: ".." },
                Log::File {
                    name: "foo.txt",
                    size: 1234
                },
                Log::Dir { name: "bar" }
            ]
        ))
    );
}
