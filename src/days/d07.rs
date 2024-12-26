// https://adventofcode.com/2022/day/7

#[allow(unused)]
const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

struct Equation {
    result: u64,
    inputs: Vec<u64>,
}

pub fn solve(input: String) -> (String, String) {
    let equations: Vec<_> = input
        .lines()
        .map(|l| {
            let mut split = l.split(":");
            let result = split.next().unwrap().parse::<u64>().unwrap();
            let inputs: Vec<_> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect();
            Equation { result, inputs }
        })
        .collect();

    // part 1
    let mut count1 = 0;
    for Equation { result, inputs } in equations.iter() {
        let n_op = inputs.len() - 1;
        let mut possible_values = Vec::with_capacity(1 << n_op);
        possible_values.push(inputs[0]);

        for &input in inputs.iter().skip(1) {
            for index in 0..possible_values.len() {
                let value = possible_values[index];
                possible_values[index] = value + input;
                possible_values.push(value * input);
            }
        }
        if possible_values.contains(result) {
            count1 += result;
        }
    }

    let mut count2 = 0;
    for Equation { result, inputs } in equations.iter() {
        let n_op = inputs.len() - 1;
        let mut possible_values = Vec::with_capacity(1 << n_op);
        possible_values.push(inputs[0]);

        for &input in inputs.iter().skip(1) {
            for index in 0..possible_values.len() {
                let value = possible_values[index];
                possible_values[index] = value + input;
                possible_values.push(value * input);
                possible_values.push(concat(value, input));
            }
        }
        if possible_values.contains(result) {
            count2 += result;
        }
    }

    (count1.to_string(), count2.to_string())
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    lhs * 10u64.pow(rhs.ilog10() + 1) + rhs
}
