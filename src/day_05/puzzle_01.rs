pub fn solve(_content: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::util::run_example_file;

    use super::*;

    #[test]
    fn test_one_example() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_example_file() {
        run_example_file(5, 0, &solve);
    }
}
