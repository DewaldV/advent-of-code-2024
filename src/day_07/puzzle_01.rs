use itertools::Itertools;
use std::iter::repeat;

#[derive(Debug)]
enum Operation {
    ADD,
    MULTIPLY,
}

pub fn solve(content: &str) -> i128 {
    let equations: Vec<(usize, Vec<usize>)> = content
        .lines()
        .map(|l| {
            let (total, values) = l.split_once(": ").expect("one : per line");
            let v: Vec<usize> = values
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect();

            (total.parse().unwrap(), v)
        })
        .collect();

    let sum_of_valid_equations: usize = equations
        .into_iter()
        .filter_map(|(total, values)| solve_eq(total, values))
        .sum();

    return sum_of_valid_equations.try_into().unwrap();
}

fn solve_eq(total: usize, values: Vec<usize>) -> Option<usize> {
    let available_ops = vec![Operation::ADD, Operation::MULTIPLY];
    let combinations = repeat(&available_ops)
        .take(values.len() - 1)
        .multi_cartesian_product();

    for ops in combinations {
        let mut sum = values[0];
        for (idx, op) in ops.iter().enumerate() {
            match *op {
                Operation::ADD => sum += values[idx + 1],
                Operation::MULTIPLY => sum *= values[idx + 1],
            }

            if sum > total {
                break;
            }
        }

        if sum == total {
            return Some(total);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::util::run_example_file;

    use super::*;

    #[test]
    fn test_example_file() {
        run_example_file(7, 3749, &solve);
    }
}
