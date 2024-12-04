pub fn solve(content: &str) -> i32 {
    let lists = lists_from(content);
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

    left.sort();
    right.sort();

    return (left, right);
}

fn distances(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let pairs = left
        .iter()
        .zip(right.iter())
        .map(|i| (i.0 - i.1).abs())
        .collect();

    return pairs;
}

#[cfg(test)]
mod tests {
    use crate::util::run_example_file;

    use super::*;

    #[test]
    fn test_example_file() {
        run_example_file(1, 11, &solve);
    }
}
