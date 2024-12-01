// https://adventofcode.com/2022/day/1

use std::collections::HashMap;

pub fn solve(input: String) -> (String, String) {
    // Parse
    let lines = input.lines();
    let location_ids = lines.map(|l| {
        let mut split = l.split_whitespace();
        let lid1 = split.next().unwrap().parse::<u32>().unwrap();
        let lid2 = split.next().unwrap().parse::<u32>().unwrap();
        (lid1, lid2)
    });
    let (list1, list2): (Vec<_>, Vec<_>) = location_ids.unzip();

    // Part 1
    let mut sorted_list1 = list1.clone();
    sorted_list1.sort();
    let mut sorted_list2 = list2.clone();
    sorted_list2.sort();

    let sum_of_diff: u32 = sorted_list1
        .iter()
        .zip(sorted_list2.iter())
        .map(|(v1, v2)| v1.abs_diff(*v2))
        .sum();

    // Part 2
    // Create a map that maps location IDs to it's frequency in the list.
    let mut frequency_map = HashMap::new();
    for value in list2.iter() {
        if let Some(f) = frequency_map.get_mut(value) {
            *f += 1;
        } else {
            frequency_map.insert(*value, 1);
        }
    }

    let sim_score: u32 = list1
        .iter()
        .map(|v| *v * frequency_map.get(v).cloned().unwrap_or(0))
        .sum();

    (sum_of_diff.to_string(), sim_score.to_string())
}
