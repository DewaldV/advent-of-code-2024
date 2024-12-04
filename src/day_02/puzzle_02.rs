use super::puzzle_01::{report_safe, reports_from};

pub fn solve(content: &str) -> i32 {
    let reports = reports_from(content);
    let safe_count = reports_safe(reports);
    return safe_count;
}

fn reports_safe(reports: Vec<Vec<i32>>) -> i32 {
    let safe_count = reports
        .iter()
        .filter(|r| try_report_with_tolerance(r))
        .count();

    return safe_count.try_into().unwrap();
}

fn try_report_with_tolerance(report: &Vec<i32>) -> bool {
    let safe = report_safe(&report);
    if safe {
        return true;
    }

    for (idx, _) in report.iter().enumerate() {
        let mut tolerated_report = report.clone();
        tolerated_report.remove(idx);

        let try_safe = report_safe(&tolerated_report);

        if try_safe {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::util::run_example_file;

    use super::*;

    #[test]
    fn test_report_safe_with_tolerance() {
        let report: Vec<i32> = vec![7, 6, 4, 2, 1];

        let safe = try_report_with_tolerance(&report);

        assert_eq!(safe, true)
    }

    #[test]
    fn test_example_file() {
        run_example_file(2, 4, &solve);
    }
}
