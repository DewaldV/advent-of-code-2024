use crate::read_to_string;
use regex::Regex;

pub fn solve(file: &str) -> i32 {
    let content = read_to_string(file);
    let result = multiply_line(&content);
    return result;
}

fn multiply_line(line: &str) -> i32 {
    let re = Regex::new(r"(?:do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\))").expect("valid regex");

    let mut enabled = true;

    let result = re.captures_iter(line).fold(0, |sum, cap| {
        let instruction = cap.get(0).unwrap().as_str();
        match instruction {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => {
                if enabled {
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    return sum + (x * y);
                }
            }
        }

        sum
    });

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
