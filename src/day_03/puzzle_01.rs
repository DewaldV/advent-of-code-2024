use regex::Regex;

pub fn solve(content: &str) -> i32 {
    let result = multiply_line(content);
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
    use crate::util::run_example_file;

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
        run_example_file(3, 161, &solve);
    }
}
