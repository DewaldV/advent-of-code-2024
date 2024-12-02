use crate::read_to_string;

pub fn solve(file: &str) -> i32 {
    let content = read_to_string(file);
    let lists = lists_from(&content);
    let distances = distances(lists.0, lists.1);

    return distances.iter().sum();
}

pub fn lists_from(content: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    content.lines().for_each(|l| {
        let line = l.split_once("   ").expect("once");
        left.push(line.0.parse().expect("number"));
        right.push(line.1.parse().expect("number"));
    });

    return (left, right);
}

fn distances(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut left_sorted = left.clone();
    left_sorted.sort();

    let mut right_sorted = right.clone();
    right_sorted.sort();

    let pairs = left_sorted
        .iter()
        .zip(right_sorted.iter())
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
        let expected_distance = 11;

        let distances = solve(example_input_file);

        assert_eq!(distances, expected_distance);
    }
}
