// https://adventofcode.com/2022/day/4
use log::{debug, trace};

/// The length of "MAS"
const LEN_MAS: usize = 3;
const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub fn solve(input: String) -> (String, String) {
    // Parse
    let haystack: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // Part 1
    let mut count1 = 0;
    for y in 0..(haystack.len()) {
        for x in 0..(haystack[y].len()) {
            count1 += search_for_xmas(x, y, &haystack);
        }
    }

    // Part 2
    let mut count2 = 0;
    for y in 1..(haystack.len() - 1) {
        for x in 1..(haystack[y].len() - 1) {
            if search_for_x_mas(x, y, &haystack) {
                count2 += 1;
            }
        }
    }

    (count1.to_string(), count2.to_string())
}

/// Searches for "MAS" both diagonal directions, centered on the
/// given (x, y).
fn search_for_x_mas(x: usize, y: usize, haystack: &Vec<Vec<char>>) -> bool {
    if haystack[y][x] != 'A' {
        return false;
    }

    // diagonal 1
    let bottom_left = haystack[y - 1][x - 1];
    let top_right = haystack[y + 1][x + 1];
    if (bottom_left != 'M' || top_right != 'S') && (bottom_left != 'S' || top_right != 'M') {
        return false;
    }
    // diagonal 2
    let bottom_right = haystack[y - 1][x + 1];
    let top_left = haystack[y + 1][x - 1];
    if (bottom_right != 'M' || top_left != 'S') && (bottom_right != 'S' || top_left != 'M') {
        return false;
    }

    true
}

/// Searches for "XMAS" in all directions
fn search_for_xmas(x: usize, y: usize, haystack: &Vec<Vec<char>>) -> u32 {
    if haystack[y][x] != 'X' {
        return 0;
    }
    // the possible directions to search
    let down = (y as isize - LEN_MAS as isize) >= 0;
    let left = (x as isize - LEN_MAS as isize) >= 0;
    let right = x + LEN_MAS < haystack[y].len();
    let up = y + LEN_MAS < haystack.len();
    trace!("At ({x}, {y}), down: {down}, left: {left}, right: {right}, up: {up}");

    // the directions we are allowed to go
    let dir_filter = [
        right,
        right && up,
        up,
        up && left,
        left,
        left && down,
        down,
        down && right,
    ];

    // iterate through all the allowed directions
    let dir_iter = DIRECTIONS
        .iter()
        .copied()
        .zip(dir_filter.into_iter())
        .filter_map(|(dir, allowed)| allowed.then_some(dir));

    let mut count = 0;
    for (dx, dy) in dir_iter {
        if check_xmas(x, y, dx, dy, haystack) {
            count += 1;
        }
    }

    count
}

/// Searches for "XMAS" in a single direction.
///
/// This assumes that the position (x, y) has an 'X'.
/// This also assumes that checking for XMAS in that direction is valid
/// (does not go out of bounds).
fn check_xmas(x: usize, y: usize, dx: isize, dy: isize, haystack: &Vec<Vec<char>>) -> bool {
    let mut x = x as isize;
    let mut y = y as isize;

    trace!("Checking XMAS at ({}, {}), going ({}, {})", x, y, dx, dy);
    // assume 'X' is at (x, y)
    for c in "MAS".chars() {
        x += dx;
        y += dy;
        trace!("\tChecking for '{}' at ({}, {})", c, x, y);

        if haystack[y as usize][x as usize] != c {
            trace!("\t\tNope! ('{}')", haystack[y as usize][x as usize]);
            return false;
        }
    }
    debug!("Found XMAS at ({x}, {y}) going ({dx}, {dy})");
    true
}
