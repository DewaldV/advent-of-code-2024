use itertools::Itertools;

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

pub fn solve(content: &str) -> i128 {
    let map: Vec<Vec<char>> = content
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut in_bounds = true;
    let mut direction = Direction::NORTH;
    let mut pos = (0, 0);

    for (r, line) in map.iter().enumerate() {
        for (c, ch) in line.iter().enumerate() {
            if *ch == '^' {
                pos = (r, c);
            }
        }
    }

    let mut visited: Vec<(usize, usize)> = Vec::new();

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
                    visited.push(pos);
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
                    visited.push(pos);
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
                    visited.push(pos);
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
                    visited.push(pos);
                }
            }
        }
    }

    let distinct = visited.iter().unique().count();

    return distinct.try_into().unwrap();
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
        run_example_file(6, 41, &solve);
    }
}
