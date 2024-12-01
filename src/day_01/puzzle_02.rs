use std::{fs, path::Path};

pub fn similarity_score_from_file(file: &str) -> i32 {
    let path = Path::new(&file);
    let content = fs::read_to_string(path).expect("expected file to exist");

    let score = score_from(&content);

    return score;
}

fn score_from(content: &str) -> i32 {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    content.lines().for_each(|l| {
        let line = l.split_once("   ").expect("once");
        left.push(line.0.parse().expect("number"));
        right.push(line.1.parse().expect("number"));
    });

    return similarity_score(left, right);
}

fn similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut score = 0;

    for l in left.iter() {
        for r in right.iter() {
            if l == r {
                score += l;
            }
        }
    }

    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_similarity_score() {
        let list_1: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let list_2: Vec<i32> = vec![4, 3, 5, 3, 9, 3];

        let expected_sim_score = 31;

        let sim_score = similarity_score(list_1, list_2);

        assert_eq!(sim_score, expected_sim_score);
    }
}
