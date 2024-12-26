// https://adventofcode.com/2022/day/8

use std::collections::{HashMap, HashSet};

use iter_tools::Itertools;

pub fn solve(input: String) -> (String, String) {
    // parse
    let mut height = 0;
    let mut width = 0;
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            if let Some(list) = antennas.get_mut(&char) {
                list.push((x as isize, y as isize));
            } else {
                antennas.insert(char, vec![(x as isize, y as isize)]);
            }
        }
        height = y as isize + 1;
        width = line.len() as isize;
    }

    // part 1
    let mut antinodes1 = HashSet::new();
    for antena_list in antennas.values() {
        for antena_pair in antena_list.iter().combinations(2) {
            let antena1 = antena_pair[0];
            let antena2 = antena_pair[1];

            let dx = antena2.0 - antena1.0;
            let dy = antena2.1 - antena1.1;

            let node1 = (antena2.0 + dx, antena2.1 + dy);
            let node2 = (antena1.0 - dx, antena1.1 - dy);

            if (0..height).contains(&node1.1) && (0..width).contains(&node1.0) {
                antinodes1.insert(node1);
            }
            if (0..height).contains(&node2.1) && (0..width).contains(&node2.0) {
                antinodes1.insert(node2);
            }
        }
    }

    // part 2
    let mut antinodes2 = HashSet::new();
    for antena_list in antennas.values() {
        for antena_pair in antena_list.iter().combinations(2) {
            let antena1 = antena_pair[0];
            let antena2 = antena_pair[1];

            let mut dx = antena2.0 - antena1.0;
            let mut dy = antena2.1 - antena1.1;

            // get the reduced version of the dx and dy
            for n in 2..((isize::max(dx, dy) as f32).sqrt() as isize + 1) {
                while dx % n == 0 && dy % n == 0 {
                    dx /= n;
                    dy /= n;
                }
            }

            // first go in the positive direction starting at antena1
            let mut current = *antena1;
            while (0..height).contains(&current.1) && (0..width).contains(&current.0) {
                antinodes2.insert(current);
                current.0 += dx;
                current.1 += dy;
            }

            // now go in the negative direction
            let mut current = *antena1;
            while (0..height).contains(&current.1) && (0..width).contains(&current.0) {
                antinodes2.insert(current);
                current.0 -= dx;
                current.1 -= dy;
            }
        }
    }

    (antinodes1.len().to_string(), antinodes2.len().to_string())
}
