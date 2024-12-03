use super::puzzle_01::multiply_instruction;
use crate::read_to_string;
use regex::Regex;

pub fn solve(file: &str) -> i32 {
    let content = read_to_string(file);
    let result = multiply_line(&content);
    return result;
}

pub fn multiply_line(line: &str) -> i32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").expect("Invalid regex");
    let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();

    let mut instructions: Vec<&str> = Vec::new();
    let mut enabled = true;

    for instruction in matches {
        match instruction {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _mul_expr => {
                if enabled {
                    instructions.push(instruction)
                }
            }
        }
    }

    let result = instructions.iter().map(|m| multiply_instruction(m)).sum();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_file() {
        let example_input_file = "src/day_03/example_input_02";
        let expected_answer = 48;

        let answer = solve(example_input_file);

        assert_eq!(answer, expected_answer);
    }
}
