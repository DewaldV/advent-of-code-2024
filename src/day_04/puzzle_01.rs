use std::collections::HashMap;

pub fn solve(content: &str) -> i32 {
    let letters: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    let mut vertical: Vec<String> = Vec::new();
    let mut left_diagonal = HashMap::new();
    let mut right_diagonal = HashMap::new();

    for (r, line) in letters.iter().enumerate() {
        for (c, ch) in line.iter().enumerate() {
            // Veritical (|)
            if r == 0 {
                vertical.insert(c, String::new())
            }

            let l = vertical.get_mut(c).unwrap();
            l.push(ch.clone());

            // Left Diagonals (\)
            let left_key = i32::try_from(r).unwrap() - i32::try_from(c).unwrap();
            if !left_diagonal.contains_key(&left_key) {
                left_diagonal.insert(left_key, String::new());
            }

            let l = left_diagonal.get_mut(&left_key).unwrap();
            l.push(ch.clone());

            // Right Diagonals (/)
            let right_key = r + c;
            if !right_diagonal.contains_key(&right_key) {
                right_diagonal.insert(right_key, String::new());
            }

            let l = right_diagonal.get_mut(&right_key).unwrap();
            l.push(ch.clone());
        }
    }

    let horizontal_count: usize = content.lines().map(|l| count_matches(&l)).sum();
    let vertical_count: usize = vertical.iter().map(|l| count_matches(&l)).sum();
    let left_diagonal_count: usize = left_diagonal.iter().map(|(_, l)| count_matches(&l)).sum();
    let right_diagonal_count: usize = right_diagonal.iter().map(|(_, l)| count_matches(&l)).sum();

    let total = horizontal_count + vertical_count + left_diagonal_count + right_diagonal_count;

    return total.try_into().unwrap();
}

fn count_matches(line: &str) -> usize {
    line.matches("XMAS").count() + line.matches("SAMX").count()
}

#[cfg(test)]
mod tests {
    use crate::util::run_example_file;

    use super::*;

    #[test]
    fn test_example_file() {
        run_example_file(4, 18, &solve);
    }
}
