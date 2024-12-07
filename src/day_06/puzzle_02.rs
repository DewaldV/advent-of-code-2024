use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

pub fn solve(content: &str) -> i32 {
    let map: Vec<Vec<char>> = content
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut pos = (0, 0);

    for (r, line) in map.iter().enumerate() {
        for (c, ch) in line.iter().enumerate() {
            if *ch == '^' {
                pos = (r, c);
            }
        }
    }

    let (_, possible_obstacles) = solvable(map.clone(), pos);

    let loop_count = possible_obstacles
        .into_iter()
        .filter(|p| *p != pos)
        .fold(0, |sum, (r, c)| {
            let mut test_map = map.clone();
            test_map[r][c] = '#';
            let (loop_detected, _) = solvable(test_map, pos);
            match loop_detected {
                true => sum + 1,
                false => sum,
            }
        });

    return loop_count;
}

fn solvable(map: Vec<Vec<char>>, starting_pos: (usize, usize)) -> (bool, Vec<(usize, usize)>) {
    let mut in_bounds = true;
    let mut direction = Direction::NORTH;
    let mut pos = starting_pos;

    let mut visited = HashMap::new();
    let mut loop_detected = false;

    while in_bounds {
        match direction {
            Direction::NORTH => {
                while in_bounds {
                    if pos.0 == 0 {
                        in_bounds = false;
                        break;
                    }

                    let ch = map[pos.0 - 1][pos.1];
                    if ch == '#' {
                        direction = Direction::EAST;
                        break;
                    }

                    pos = (pos.0 - 1, pos.1);
                    let key = (pos.0, pos.1, direction);
                    if visited.contains_key(&key) {
                        loop_detected = true;
                        in_bounds = false;
                    }

                    visited.insert(key, true);
                }
            }
            Direction::SOUTH => {
                while in_bounds {
                    if pos.0 == map[pos.1].len() - 1 {
                        in_bounds = false;
                        break;
                    }

                    let ch = map[pos.0 + 1][pos.1];
                    if ch == '#' {
                        direction = Direction::WEST;
                        break;
                    }

                    pos = (pos.0 + 1, pos.1);
                    let key = (pos.0, pos.1, direction);
                    if visited.contains_key(&key) {
                        loop_detected = true;
                        in_bounds = false;
                    }

                    visited.insert(key, true);
                }
            }
            Direction::EAST => {
                while in_bounds {
                    if pos.1 == map[pos.0].len() - 1 {
                        in_bounds = false;
                        break;
                    }

                    let ch = map[pos.0][pos.1 + 1];
                    if ch == '#' {
                        direction = Direction::SOUTH;
                        break;
                    }

                    pos = (pos.0, pos.1 + 1);
                    let key = (pos.0, pos.1, direction);
                    if visited.contains_key(&key) {
                        loop_detected = true;
                        in_bounds = false;
                    }

                    visited.insert(key, true);
                }
            }
            Direction::WEST => {
                while in_bounds {
                    if pos.1 == 0 {
                        in_bounds = false;
                        break;
                    }

                    let ch = map[pos.0][pos.1 - 1];
                    if ch == '#' {
                        direction = Direction::NORTH;
                        break;
                    }

                    pos = (pos.0, pos.1 - 1);
                    let key = (pos.0, pos.1, direction);
                    if visited.contains_key(&key) {
                        loop_detected = true;
                        in_bounds = false;
                    }

                    visited.insert(key, true);
                }
            }
        }
    }

    let visited_list: Vec<(usize, usize)> = visited
        .into_keys()
        .map(|(r, c, _)| (r, c))
        .unique()
        .collect();

    return (loop_detected, visited_list);
}

#[cfg(test)]
mod tests {
    use crate::util::run_example_file;

    use super::*;

    #[test]
    fn test_one_example() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_example_file() {
        run_example_file(6, 6, &solve);
    }
}
