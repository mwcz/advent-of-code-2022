use argh::FromArgs;
use std::fs::read_to_string;

fn main() {
    let args: Args = argh::from_env();

    let input_file = if args.input.is_some() {
        args.input.unwrap()
    } else if args.example {
        format!("./examples/d{}", args.day)
    } else {
        format!("./input/d{}", args.day)
    };

    let input = read_to_string(input_file).expect("couldn't read input file");

    match (args.day, args.part) {
        (1, 1) => {
            let parsed = aoc2022::d1::parse(input);
            let output = aoc2022::d1::part1(parsed);
            println!("{output}");
        }
        (1, 2) => {
            let parsed = aoc2022::d1::parse(input);
            let output = aoc2022::d1::part2(parsed);
            println!("{output}");
        }
        (2, 1) => {
            let parsed = aoc2022::d2::parse(input);
            let output = aoc2022::d2::part1(parsed);
            println!("{output}");
        }
        (2, 2) => {
            let parsed = aoc2022::d2::parse(input);
            let output = aoc2022::d2::part2(parsed);
            println!("{output}");
        }
        (3, 1) => {
            let parsed = aoc2022::d3::parse(input);
            let output = aoc2022::d3::part1(parsed);
            println!("{output}");
        }
        (3, 2) => {
            let parsed = aoc2022::d3::parse(input);
            let output = aoc2022::d3::part2(parsed);
            println!("{output}");
        }
        (4, 1) => {
            let parsed = aoc2022::d4::parse(input);
            let output = aoc2022::d4::part1(parsed);
            println!("{output}");
        }
        (4, 2) => {
            let parsed = aoc2022::d4::parse(input);
            let output = aoc2022::d4::part2(parsed);
            println!("{output}");
        }
        (5, 1) => {
            let parsed = aoc2022::d5::parse(input);
            let output = aoc2022::d5::part1(parsed);
            println!("{output}");
        }
        (5, 2) => {
            let parsed = aoc2022::d5::parse(input);
            let output = aoc2022::d5::part2(parsed);
            println!("{output}");
        }
        (6, 1) => {
            let parsed = aoc2022::d6::parse(input);
            let output = aoc2022::d6::part1(parsed);
            println!("{output}");
        }
        (6, 2) => {
            let parsed = aoc2022::d6::parse(input);
            let output = aoc2022::d6::part2(parsed);
            println!("{output}");
        }
        (7, 1) => {
            let parsed = aoc2022::d7::parse(input);
            let output = aoc2022::d7::part1(parsed);
            println!("{output}");
        }
        (7, 2) => {
            let parsed = aoc2022::d7::parse(input);
            let output = aoc2022::d7::part2(parsed);
            println!("{output}");
        }
        (8, 1) => {
            let parsed = aoc2022::d8::parse(input);
            let output = aoc2022::d8::part1(parsed);
            println!("{output}");
        }
        (8, 2) => {
            let parsed = aoc2022::d8::parse(input);
            let output = aoc2022::d8::part2(parsed);
            println!("{output}");
        }
        (9, 1) => {
            let parsed = aoc2022::d9::parse(input);
            let output = aoc2022::d9::part1(parsed);
            println!("{output}");
        }
        (9, 2) => {
            let parsed = aoc2022::d9::parse(input);
            let output = aoc2022::d9::part2(parsed);
            println!("{output}");
        }
        (10, 1) => {
            let parsed = aoc2022::d10::parse(input);
            let output = aoc2022::d10::part1(parsed);
            println!("{output}");
        }
        (10, 2) => {
            let parsed = aoc2022::d10::parse(input);
            let output = aoc2022::d10::part2(parsed);
            println!("{output}");
        }
        (11, 1) => {
            let parsed = aoc2022::d11::parse(input);
            let output = aoc2022::d11::part1(parsed);
            println!("{output}");
        }
        (11, 2) => {
            let parsed = aoc2022::d11::parse(input);
            let output = aoc2022::d11::part2(parsed);
            println!("{output}");
        }
        (12, 1) => {
            let parsed = aoc2022::d12::parse(input);
            let output = aoc2022::d12::part1(parsed);
            println!("{output}");
        }
        (12, 2) => {
            let parsed = aoc2022::d12::parse(input);
            let output = aoc2022::d12::part2(parsed);
            println!("{output}");
        }
        (13, 1) => {
            let parsed = aoc2022::d13::parse(input);
            let output = aoc2022::d13::part1(parsed);
            println!("{output}");
        }
        (13, 2) => {
            let parsed = aoc2022::d13::parse(input);
            let output = aoc2022::d13::part2(parsed);
            println!("{output}");
        }
        (14, 1) => {
            let parsed = aoc2022::d14::parse(input);
            let output = aoc2022::d14::part1(parsed);
            println!("{output}");
        }
        (14, 2) => {
            let parsed = aoc2022::d14::parse(input);
            let output = aoc2022::d14::part2(parsed);
            println!("{output}");
        }
        (15, 1) => {
            let parsed = aoc2022::d15::parse(input);
            let output = aoc2022::d15::part1(parsed);
            println!("{output}");
        }
        (15, 2) => {
            let parsed = aoc2022::d15::parse(input);
            let output = aoc2022::d15::part2(parsed);
            println!("{output}");
        }
        (16, 1) => {
            let parsed = aoc2022::d16::parse(&input);
            let output = aoc2022::d16::part1(parsed);
            println!("{output}");
        }
        (16, 2) => {
            let parsed = aoc2022::d16::parse(&input);
            let output = aoc2022::d16::part2(parsed);
            println!("{output}");
        }
        (17, 1) => {
            let parsed = aoc2022::d17::parse(input);
            let output = aoc2022::d17::part1(parsed);
            println!("{output}");
        }
        (17, 2) => {
            let parsed = aoc2022::d17::parse(input);
            let output = aoc2022::d17::part2(parsed);
            println!("{output}");
        }
        (18, 1) => {
            let parsed = aoc2022::d18::parse(input);
            let output = aoc2022::d18::part1(parsed);
            println!("{output}");
        }
        (18, 2) => {
            let parsed = aoc2022::d18::parse(input);
            let output = aoc2022::d18::part2(parsed);
            println!("{output}");
        }
        (19, 1) => {
            let parsed = aoc2022::d19::parse(input);
            let output = aoc2022::d19::part1(parsed);
            println!("{output}");
        }
        (19, 2) => {
            let parsed = aoc2022::d19::parse(input);
            let output = aoc2022::d19::part2(parsed);
            println!("{output}");
        }
        (20, 1) => {
            let parsed = aoc2022::d20::parse(input);
            let output = aoc2022::d20::part1(parsed);
            println!("{output}");
        }
        (20, 2) => {
            let parsed = aoc2022::d20::parse(input);
            let output = aoc2022::d20::part2(parsed);
            println!("{output}");
        }
        (21, 1) => {
            let parsed = aoc2022::d21::parse(input);
            let output = aoc2022::d21::part1(parsed);
            println!("{output}");
        }
        (21, 2) => {
            let parsed = aoc2022::d21::parse(input);
            let output = aoc2022::d21::part2(parsed);
            println!("{output}");
        }
        (22, 1) => {
            let parsed = aoc2022::d22::parse(input);
            let output = aoc2022::d22::part1(parsed);
            println!("{output}");
        }
        (22, 2) => {
            let parsed = aoc2022::d22::parse(input);
            let output = aoc2022::d22::part2(parsed);
            println!("{output}");
        }
        (23, 1) => {
            let parsed = aoc2022::d23::parse(input);
            let output = aoc2022::d23::part1(parsed);
            println!("{output}");
        }
        (23, 2) => {
            let parsed = aoc2022::d23::parse(input);
            let output = aoc2022::d23::part2(parsed);
            println!("{output}");
        }
        (24, 1) => {
            let parsed = aoc2022::d24::parse(input);
            let output = aoc2022::d24::part1(parsed);
            println!("{output}");
        }
        (24, 2) => {
            let parsed = aoc2022::d24::parse(input);
            let output = aoc2022::d24::part2(parsed);
            println!("{output}");
        }
        (25, 1) => {
            let parsed = aoc2022::d25::parse(input);
            let output = aoc2022::d25::part1(parsed);
            println!("{output}");
        }
        (25, 2) => {
            let parsed = aoc2022::d25::parse(input);
            let output = aoc2022::d25::part2(parsed);
            println!("{output}");
        }
        _ => unimplemented!(),
    }
}

/// The CLI arguments allowed.
#[derive(FromArgs, Debug)]
struct Args {
    /// specifies the day
    #[argh(option, short = 'd')]
    day: u8,

    /// specifies the part
    #[argh(option, short = 'p', default = "1")]
    part: u8,

    /// use the day's example input from examples/
    #[argh(switch, short = 'e')]
    example: bool,

    /// specify an alternate input file
    #[argh(option, short = 'i')]
    input: Option<String>,
}
