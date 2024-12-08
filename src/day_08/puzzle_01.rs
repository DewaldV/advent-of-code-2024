use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Pos {
    r: i32,
    c: i32,
}

impl Pos {
    fn antinode(&self, other: &Pos, max_r: i32, max_c: i32) -> Option<Pos> {
        if other.r == self.r && other.c == self.c {
            return None;
        }

        let r_diff = self.r - other.r;
        let c_diff = self.c - other.c;

        let r = other.r + (r_diff * -1);
        let c = other.c + (c_diff * -1);

        if r < 0 || c < 0 || r >= max_r || c >= max_c {
            return None;
        }

        Some(Pos { r, c })
    }
}

pub fn solve(content: &str) -> i128 {
    let map: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    let mut antennae: HashMap<char, Vec<Pos>> = HashMap::new();

    let (max_r, max_c) = (map.len() as i32, map.first().unwrap().len() as i32);

    for (r, line) in content.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennae.entry(ch).or_insert_with(Vec::new).push(Pos {
                    r: r as i32,
                    c: c as i32,
                });
            }
        }
    }

    let mut antinodes: HashMap<Pos, bool> = HashMap::new();

    for ants in antennae.values() {
        let combinations = ants.iter().cartesian_product(ants);
        for pair in combinations {
            if let Some(antinode) = pair.0.antinode(pair.1, max_r, max_c) {
                antinodes.insert(antinode, true);
            }
        }
    }

    return antinodes.keys().len().try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::util::{read_to_string, run_example_file};

    use super::*;

    #[test]
    fn test_one_example() {
        let content = read_to_string("src/day_08/example_input_02");
        let answer = solve(&content);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_example_file() {
        run_example_file(8, 14, &solve);
    }
}
