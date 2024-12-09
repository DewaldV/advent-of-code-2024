use crate::util::run_day;

pub mod puzzle_01;
pub mod puzzle_02;

pub fn run() {
    run_day(9, &puzzle_01::solve, &puzzle_02::solve)
}
