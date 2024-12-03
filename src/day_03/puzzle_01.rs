use regex::Regex;

use crate::read_to_string;

pub fn solve(file: &str) -> i32 {
    let content = read_to_string(file);
    let result = multiply_line(&content);
    return result;
}

fn multiply_line(line: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("valid regex");
    let result = re
        .captures_iter(line)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_example() {
        let line = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected_answer = 161;

        let answer = multiply_line(line);

        assert_eq!(answer, expected_answer);
    }

    #[test]
    fn test_example_file() {
        let example_input_file = "src/day_03/example_input_01";
        let expected_answer = 161;

        let answer = solve(example_input_file);

        assert_eq!(answer, expected_answer);
    }
}
