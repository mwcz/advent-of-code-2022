use aoc_runner_derive::aoc;
use num_bigint::BigUint;
use once_cell::sync::Lazy;

// prime factor approach won't work because even u128 would be needed to hold the upper limit of
// rucksack products :(
// hm, cuviper's num-bigint would help, maybe I'll go for that.  seems fun.

#[rustfmt::skip]
const PRIMES_U32: [u32; 52] = [
/*  a  b  c  d   e   f   g   h   i   j   k   l   m   n   o   p   q   r   s   t   u   v   w   x   y */
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
/*    z    A    B    C    D    E    F    G    H    I    J    K    L    M    N    O    P    Q    R */
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
/*    S    T    U    V    W    X    Y    Z */
    197, 199, 211, 223, 227, 229, 233, 239,
];
static PRIMES: Lazy<[BigUint; 52]> = Lazy::new(|| PRIMES_U32.map(BigUint::from));

/// Map the chars 'a'..'z' and then 'A'..'Z' to the first 52 prime numbers and return an Item
/// containing the prime and the priority of the letter.
fn prime(c: char) -> Item<'static> {
    const CAPS_OFFSET: usize = 65 - 26;
    const LOWER_OFFSET: usize = 97;

    let offset = match c {
        'a'..='z' => c as usize - LOWER_OFFSET,
        'A'..='Z' => c as usize - CAPS_OFFSET,
        _ => panic!("non-alpha character {c}"),
    };
    let prime = &PRIMES[offset];
    let priority = offset + 1;

    Item::new(prime, priority)
}

/// Each rucksack is represented as two compartments.
type Rucksacks<'int> = Vec<Rucksack<'int>>;
type Rucksack<'int> = (Compartment<'int>, Compartment<'int>);
/// Each compartment is a series of u32s (which have been converted into primes).
type Compartment<'int> = Vec<Item<'int>>;
/// Each item is a prime number, plus its corresponding priority (to avoid having to look up
/// priority later on).
#[derive(Debug, PartialEq)]
struct Item<'int> {
    prime: &'int BigUint,
    priority: usize,
}

impl<'int> Item<'int> {
    fn new(prime: &'int BigUint, priority: usize) -> Self {
        Self { prime, priority }
    }
}

/// Parse the input.  Not using the generator because my parsing involves references.  See
/// cargo-aoc issue #20 https://github.com/gobanos/cargo-aoc/issues/20
fn part1_parse(input: &str) -> Rucksacks {
    input
        .lines()
        .map(|line| {
            let comp_strs = line.split_at(line.len() / 2);
            let comp0_primes: Compartment = comp_strs.0.chars().map(prime).collect();
            let comp1_primes: Compartment = comp_strs.1.chars().map(prime).collect();
            (comp0_primes, comp1_primes)
        })
        .collect()
}

/// Parse the input tailored for part 2.
fn part2_parse(input: &str) -> Vec<Vec<Compartment>> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| -> Vec<Compartment> {
            chunk
                .iter()
                .map(|line| line.chars().map(prime).collect::<Compartment>())
                .collect()
            // let comp0_primes: Compartment = comp_strs.0.chars().map(prime).collect();
            // let comp1_primes: Compartment = comp_strs.1.chars().map(prime).collect();
            // (comp0_primes, comp1_primes)
        })
        .collect()
    // .map(|line| {
    //     let comp_strs = line.split_at(line.len() / 2);
    //     let comp0_primes: Compartment = comp_strs.0.chars().map(prime).collect();
    //     let comp1_primes: Compartment = comp_strs.1.chars().map(prime).collect();
    //     (comp0_primes, comp1_primes)
    // })
    // .collect()
}

/// For a given compartment, find the product of all the prime numbers contained in each Item.
fn product(items: &Compartment) -> BigUint {
    items
        .iter()
        .fold(BigUint::from(1u32), |acc, item| acc * item.prime)
}

#[aoc(day3, part1)]
fn part1_solve(input: &str) -> usize {
    let rucksacks = part1_parse(input);

    let mut priority_sum = 0;

    for rucksack in rucksacks.iter() {
        // find the product of compartment 1's prime numbers
        let comp1_product = product(&rucksack.1);

        // check each of compartment 0's prime numbers to see if they evenly divide into
        // compartment 1's product.  if they do, that's the prime corresponding to the item in both
        // compartments.
        for item in &rucksack.0 {
            // if the prime evently divides into compartment 1's product, then the letter exists in
            // that compartment too, so it's what we're looking for.
            if &comp1_product % item.prime == BigUint::from(0u32) {
                priority_sum += item.priority;
                break;
            }
        }
    }

    priority_sum
}

#[aoc(day3, part2)]
fn part2_solve(input: &str) -> usize {
    let compartments = part2_parse(input);

    let mut priority_sum = 0;

    for triplet in compartments {
        let comp0_product = product(&triplet[0]);
        let comp1_product = product(&triplet[1]);

        for item in &triplet[2] {
            // if the prime evently divides into compartment 1's product, then the letter exists in
            // that compartment too, so it's what we're looking for.
            if &comp0_product % item.prime == BigUint::from(0u32) && &comp1_product % item.prime == BigUint::from(0u32) {
                priority_sum += item.priority;
                break;
            }
        }
    }

    priority_sum
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn prime_test() {
        assert_eq!(prime('A'), Item::new(&BigUint::from(103u32), 27), "A");
        assert_eq!(prime('Z'), Item::new(&BigUint::from(239u32), 52), "Z");
        assert_eq!(prime('a'), Item::new(&BigUint::from(2u32), 1), "a");
        assert_eq!(prime('z'), Item::new(&BigUint::from(101u32), 26), "z");
    }

    #[test]
    fn part1_solve_test() {
        assert_eq!(
            part1_solve(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            157
        );
    }

    #[test]
    fn part2_solve_test() {
        assert_eq!(
            part2_solve(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            70
        );
    }
}
