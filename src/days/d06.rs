// https://adventofcode.com/2022/day/6

#[allow(unused)]
const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
enum Direction {
    Right = 0,
    #[default]
    Up = 1,
    Left = 2,
    Down = 3,
}
impl Direction {
    // Gets the (dx, dy) coordinates that move in this direction.
    fn coords(self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::Up => (0, -1), // up on the map, is a lower index
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
        }
    }

    /// Rotates 90 degrees.
    #[must_use]
    fn rotate(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Tile {
    Obsticle,
    GuardStarting,
    Visited(u8),
    Empty,
}
impl Tile {
    /// Parses from a character.
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile::Obsticle),
            '^' => Some(Tile::GuardStarting),
            'X' => Some(Tile::Visited(0)),
            '.' => Some(Tile::Empty),
            _ => None,
        }
    }

    fn to_char(self) -> char {
        match self {
            Tile::Obsticle => '#',
            Tile::GuardStarting => '^',
            Tile::Visited(_) => 'X',
            Tile::Empty => '.',
        }
    }
}

pub fn solve(input: String) -> (String, String) {
    // Parse
    let mut guard_pos = None;
    let map: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = Tile::from_char(c).expect(&format!("Invalid char: {}", c));
                    if tile == Tile::GuardStarting {
                        if guard_pos.is_some() {
                            panic!("Two guards?!?");
                        }
                        guard_pos = Some((x, y));
                    }
                    tile
                })
                .collect()
        })
        .collect();
    let starting_guard_pos = guard_pos
        .map(|(x, y)| (x as isize, y as isize))
        .expect("No guard?!?");
    let mut guard_pos = starting_guard_pos;
    let mut dir = Direction::Up;

    // Part 1
    let mut p1_map = map.clone();
    loop {
        let next_guard_pos = move_guard(guard_pos, dir);
        if !in_map(next_guard_pos, &p1_map) {
            break;
        }

        let tile = p1_map[next_guard_pos.1 as usize][next_guard_pos.0 as usize];
        match tile {
            Tile::Obsticle => {
                dir = dir.rotate();
                // do not walk forward
                continue;
            }
            Tile::Visited(_) | Tile::GuardStarting => {}
            Tile::Empty => {
                p1_map[next_guard_pos.1 as usize][next_guard_pos.0 as usize] = Tile::Visited(0);
            }
        }

        // walk forward
        guard_pos = next_guard_pos;
    }

    // count visited locations
    let visited_locations: Vec<_> = p1_map
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(move |(column_index, t)| (row_index, column_index, t))
        })
        .flatten()
        .filter_map(|(row, column, t)| matches!(*t, Tile::Visited(_)).then_some((row, column)))
        .collect();

    let location_count = visited_locations.len() + 1; // plus 1 for the guard starting location.

    // Part 2
    let mut loop_count = 0;
    for y_location in 0..map.len() {
        for x_location in 0..map[y_location].len() {
            // try adding an object to all visited locations to see if it causes a loop
            let mut modified_map = map.clone();
            modified_map[y_location][x_location] = Tile::Obsticle;
            if check_for_loop(modified_map, starting_guard_pos) {
                loop_count += 1;
            }
        }
    }

    (location_count.to_string(), loop_count.to_string())
}

/// Check if the given map causes the guard to travel in a loop and never exit.
fn check_for_loop(mut map: Vec<Vec<Tile>>, mut guard_pos: (isize, isize)) -> bool {
    map[guard_pos.1 as usize][guard_pos.0 as usize] = Tile::Visited(1 << (Direction::Up as u8));

    let mut dir = Direction::Up;
    loop {
        let next_guard_pos = move_guard(guard_pos, dir);
        if !in_map(next_guard_pos, &map) {
            return false;
        }

        let tile = map
            .get_mut(next_guard_pos.1 as usize)
            .unwrap()
            .get_mut(next_guard_pos.0 as usize)
            .unwrap();
        match *tile {
            Tile::Obsticle => {
                dir = dir.rotate();

                // do not walk forward
                continue;
            }
            Tile::GuardStarting => {
                panic!("`GuardStarting` tile should have been replaced by a `Visited`")
            }
            Tile::Visited(dir_mask) => {
                let new_dir_mask = dir_mask | (1 << (dir as u8));
                if dir_mask == new_dir_mask {
                    // loop!
                    return true;
                } else {
                    *tile = Tile::Visited(new_dir_mask);
                }
            }
            Tile::Empty => {
                *tile = Tile::Visited(1 << (dir as u8));
            }
        }

        // walk forward
        guard_pos = next_guard_pos;
    }
}

#[inline]
fn in_map(pos: (isize, isize), map: &Vec<Vec<Tile>>) -> bool {
    let (x, y) = pos;
    let y_len = map.len() as isize;
    (0..y_len).contains(&y) && (0..(map[y as usize].len() as isize)).contains(&x)
}

#[inline]
fn move_guard(guard_pos: (isize, isize), guard_dir: Direction) -> (isize, isize) {
    let (dx, dy) = guard_dir.coords();
    (guard_pos.0 + dx, guard_pos.1 + dy)
}

/// Prints the map
///
/// For debugging.
#[allow(unused)]
fn print_map(map: &Vec<Vec<Tile>>) {
    let str = map
        .iter()
        .map(|row| row.iter().map(|t| t.to_char()).collect::<String>() + "\n")
        .collect::<String>();
    println!("{}", str);
}
