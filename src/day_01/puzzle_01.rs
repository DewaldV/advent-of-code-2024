use std::{fs, path::Path};

pub fn distance_from_file(file: &str) -> i32 {
    let distances = distances_from_file(file);
    return distances.iter().sum();
}

fn distances_from_file(file: &str) -> Vec<i32> {
    let path = Path::new(&file);
    let content = fs::read_to_string(path).expect("expected file to exist");

    let distances = distances_from(&content);

    return distances;
}

fn distances_from(content: &str) -> Vec<i32> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    content.lines().for_each(|l| {
        let line = l.split_once("   ").expect("once");
        left.push(line.0.parse().expect("number"));
        right.push(line.1.parse().expect("number"));
    });

    return distances(left, right);
}

fn distances(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
    left.sort();
    right.sort();

    let pairs = left
        .iter()
        .zip(right.iter())
        .map(|i| (i.0 - i.1).abs())
        .collect();

    return pairs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_list_distances() {
        let list_1: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let list_2: Vec<i32> = vec![4, 3, 5, 3, 9, 3];

        let expected_distances: Vec<i32> = vec![2, 1, 0, 1, 2, 5];

        let paired_lists = distances(list_1, list_2);

        assert_eq!(paired_lists, expected_distances);
    }

    #[test]
    fn pair_lists_from_file() {
        let example_input_file = "src/day_01/example_input_01";
        let expected_distances: Vec<i32> = vec![2, 1, 0, 1, 2, 5];

        let distances = distances_from_file(example_input_file);

        assert_eq!(distances, expected_distances);
    }
}
