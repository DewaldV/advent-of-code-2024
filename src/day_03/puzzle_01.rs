use regex::Regex;

use crate::read_to_string;

pub fn solve(file: &str) -> i32 {
    let content = read_to_string(file);
    let result = multiply_line(&content);
    return result;
}

pub fn multiply_line(line: &str) -> i32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").expect("Invalid regex");
    let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();

    let result = matches.iter().map(|m| multiply_instruction(m)).sum();

    return result;
}

pub fn multiply_instruction(i: &str) -> i32 {
    let args = i.split('(').last().unwrap().trim_end_matches(')');
    let result: i32 = args
        .split(',')
        .map(|a| a.parse::<i32>().unwrap())
        .reduce(|acc, e| acc * e)
        .unwrap();
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
