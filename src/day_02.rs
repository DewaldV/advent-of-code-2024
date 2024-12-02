pub mod puzzle_01;
pub mod puzzle_02;

pub fn run() {
    println!("\nDay 02 Answers");
    println!("--------------");

    let file = "src/day_02/input_01";

    let answer1 = puzzle_01::solve(file);

    println!("Puzzle 01: {}", answer1);

    let file = "src/day_02/input_01";

    let answer2 = puzzle_02::solve(file);

    println!("Puzzle 02: {}", answer2);
    println!("--------------");
}
