pub fn solve(content: &str) -> i32 {
    let mut count = 0;

    let letters: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    for (r, line) in letters.iter().enumerate() {
        for (c, ch) in line.iter().enumerate() {
            if r == 0 || c == 0 || r == letters.len() - 1 || c == line.len() - 1 {
                continue;
            }

            if *ch == 'A' {
                let top_left = letters[r - 1][c - 1];
                let top_right = letters[r - 1][c + 1];
                let bottom_left = letters[r + 1][c - 1];
                let bottom_right = letters[r + 1][c + 1];

                // Left Diagonal (\)
                let mut left = String::new();
                left.push(top_left);
                left.push(bottom_right);

                // Right Diagonal (/)
                let mut right = String::new();
                right.push(top_right);
                right.push(bottom_left);

                if (left == "MS" || left == "SM") && (right == "MS" || right == "SM") {
                    count += 1;
                }
            }
        }
    }

    return count.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::read_to_string;

    use super::*;

    #[test]
    fn test_example_file() {
        let example_input_file = "src/day_04/example_input_01";
        let example_input = read_to_string(&example_input_file);
        let expected_answer = 9;

        let answer = solve(&example_input);

        assert_eq!(answer, expected_answer);
    }
}
