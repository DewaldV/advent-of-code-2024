use crate::read_to_string;

use super::puzzle_01::{report_safe, reports_from};

pub fn count_safe_reports(file: &str) -> i32 {
    let content = read_to_string(file);
    let reports = reports_from(&content);
    let safe_count = reports_safe(reports);
    return safe_count;
}

fn reports_safe(reports: Vec<Vec<i32>>) -> i32 {
    let safe_count = reports
        .iter()
        .map(|r| try_report_with_tolerance(r.to_vec()))
        .filter(|r| *r)
        .count();

    return safe_count.try_into().unwrap();
}

fn try_report_with_tolerance(report: Vec<i32>) -> bool {
    let safe = report_safe(&report);

    if !safe {
        for (idx, _) in report.iter().enumerate() {
            let mut tolerated_report = report.clone();
            tolerated_report.remove(idx);
            let try_safe = report_safe(&tolerated_report);
            if try_safe {
                return try_safe;
            }
        }
    }

    return safe;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_safe_with_tolerance() {
        let report: Vec<i32> = vec![7, 6, 4, 2, 1];

        let safe = try_report_with_tolerance(report);

        assert_eq!(safe, true)
    }

    #[test]
    fn test_example_reports_safe() {
        let example_input_file = "src/day_02/example_input_01";
        let expected_safe_count = 4;

        let safe_count = count_safe_reports(example_input_file);

        assert_eq!(safe_count, expected_safe_count);
    }
}
