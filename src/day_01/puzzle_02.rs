use super::puzzle_01::lists_from;

pub fn solve(content: &str) -> i32 {
    let lists = lists_from(content);
    let score = similarity_score(lists.0, lists.1);

    return score;
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
    use crate::util::run_example_file;

    use super::*;

    #[test]
    fn solve_similarity_score() {
        let list_1: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let list_2: Vec<i32> = vec![4, 3, 5, 3, 9, 3];

        let expected_sim_score = 31;

        let sim_score = similarity_score(list_1, list_2);

        assert_eq!(sim_score, expected_sim_score);
    }

    #[test]
    fn test_example_file() {
        run_example_file(1, 31, &solve);
    }
}
