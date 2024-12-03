// https://adventofcode.com/2022/day/3

use regex::Regex;

// const REGEX: &str = r#"(?:mul\((\d\d?\d?),(\d\d?\d?)\))|(?:(do\(\)|don\'t\(\)))"#;
const REGEX: &str = r#"(?:mul\((\d\d?\d?),(\d\d?\d?)\))|(?:(do\(\)|don\'t\(\))())"#;

/// Any possible instruction.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(u64, u64),
}

pub fn solve(input: String) -> (String, String) {
    // Parse
    let reg = Regex::new(REGEX).expect("failed to parse regex");
    let muls: Vec<Instruction> = reg
        .captures_iter(&input)
        .map(|c| c.extract().1)
        .map(|[s1, s2]| match (s1.parse().ok(), s2.parse().ok(), s1) {
            (Some(v1), Some(v2), _) => Instruction::Mul(v1, v2),
            (_, _, "do()") => Instruction::Do,
            (_, _, "don't()") => Instruction::Dont,
            _ => panic!("Invalid instruction: \"{:?}\"", (s1, s2)),
        })
        .collect();

    // Part 1
    let sum1: u64 = muls
        .iter()
        .filter_map(|i| {
            if let Instruction::Mul(v1, v2) = i {
                Some(v1 * v2)
            } else {
                None
            }
        })
        .sum();

    // Part 2
    let sum2 = muls
        .iter()
        .fold((true, 0), |(mut enabled, mut sum), i| {
            match i {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                Instruction::Mul(v1, v2) => {
                    if enabled {
                        sum += v1 * v2
                    }
                }
            }
            (enabled, sum)
        })
        .1;

    (sum1.to_string(), sum2.to_string())
}
