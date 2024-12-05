use std::collections::HashMap;

pub fn solve(content: &str) -> i32 {
    let split_content = content
        .split_once("\n\n")
        .expect("one instance of double new line");

    let mut rules = HashMap::new();
    split_content.0.lines().for_each(|l| {
        let rule = l.split_once('|').expect("pipe seperated");
        if !rules.contains_key(rule.0) {
            rules.insert(rule.0, Vec::new());
        }

        rules.get_mut(rule.0).unwrap().push(rule.1);
    });

    let updates: Vec<Vec<&str>> = split_content
        .1
        .lines()
        .map(|l| l.split(',').collect())
        .collect();

    let middle_sum = updates
        .iter()
        .filter(|update| {
            update.is_sorted_by(|a, b| {
                if let Some(r) = rules.get(b) {
                    return !r.contains(&a);
                }
                true
            })
        })
        .map(|update| {
            let mid = update.len() / 2;
            let mid_val = update.get(mid).unwrap();
            (*mid_val).parse::<i32>().unwrap()
        })
        .sum();

    return middle_sum;
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
        run_example_file(5, 143, &solve);
    }
}
