use crate::read_to_string;

pub fn solve(file: &str) -> i32 {
    let content = read_to_string(file);
    let reports = reports_from(&content);
    let safe_count = reports_safe(reports);
    return safe_count;
}

pub fn reports_from(content: &str) -> Vec<Vec<i32>> {
    let reports: Vec<Vec<i32>> = content
        .lines()
        .map(|l| l.split_whitespace().map(|l| l.parse().unwrap()).collect())
        .collect();
    return reports;
}

fn reports_safe(reports: Vec<Vec<i32>>) -> i32 {
    let safe_count = reports.iter().filter(|r| report_safe(r)).count();

    return safe_count.try_into().unwrap();
}

pub fn report_safe(report: &Vec<i32>) -> bool {
    let mut next_level_it = report.iter();
    next_level_it.next();

    let diffs: Vec<i32> = report
        .iter()
        .filter_map(|level| {
            let next_level = next_level_it.next();
            match next_level {
                Some(next_level) => Some(level - next_level),
                None => None,
            }
        })
        .collect();

    let first_diff = diffs.first().expect("at least one level");

    let levels_safe = diffs.iter().all(|d| diff_safe(first_diff.is_positive(), d));

    return levels_safe;
}

fn diff_safe(postive: bool, diff: &i32) -> bool {
    return diff.is_positive() == postive && diff.abs() >= 1 && diff.abs() <= 3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_safe() {
        let report: Vec<i32> = vec![7, 6, 4, 2, 1];

        let safe = report_safe(&report);

        assert_eq!(safe, true)
    }

    #[test]
    fn test_example_reports_safe() {
        let example_input_file = "src/day_02/example_input_01";
        let expected_safe_count = 2;

        let safe_count = solve(example_input_file);

        assert_eq!(safe_count, expected_safe_count);
    }
}
