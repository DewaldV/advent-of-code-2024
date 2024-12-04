pub fn solve(content: &str) -> i32 {
    // let reports = reports_from(&content);
    // let safe_count = reports_safe(reports);
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::read_to_string;

    use super::*;

    #[test]
    fn test_one_example() {
        // let report: Vec<i32> = vec![7, 6, 4, 2, 1];
        // let safe = report_safe(&report);
        // assert_eq!(safe, true)
    }

    #[test]
    fn test_example_file() {
        let example_input_file = "src/day_05/example_input_01";
        let example_input = read_to_string(&example_input_file);
        let expected_answer = 0;

        let answer = solve(&example_input);

        assert_eq!(answer, expected_answer);
    }
}
