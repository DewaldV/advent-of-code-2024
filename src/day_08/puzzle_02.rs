use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Pos {
    r: i32,
    c: i32,
}

// 1: (3, 4)
// 2: (5, 5)
//

impl Pos {
    fn antinodes(&self, other: &Pos, max_r: i32, max_c: i32) -> Vec<Pos> {
        let mut nodes = Vec::new();

        if other.r == self.r && other.c == self.c {
            return nodes;
        }

        nodes.push(self.clone());
        nodes.push(other.clone());

        let mut antinode: Pos = other.clone();

        let r_diff = self.r - antinode.r;
        let c_diff = self.c - antinode.c;

        loop {
            let r = antinode.r + (r_diff * -1);
            let c = antinode.c + (c_diff * -1);

            antinode = Pos { r, c };

            if antinode.r < 0 || antinode.c < 0 || antinode.r >= max_r || antinode.c >= max_c {
                break;
            }

            nodes.push(antinode.clone());
        }

        nodes
    }
}

pub fn solve(content: &str) -> i128 {
    let map: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    if map.is_empty() {
        return 0;
    }

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
            let new_nodes = pair.0.antinodes(pair.1, max_r, max_c);
            for a in new_nodes {
                antinodes.insert(a, true);
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
        assert_eq!(answer, 5);
    }

    #[test]
    fn test_example_file() {
        run_example_file(8, 34, &solve);
    }
}
