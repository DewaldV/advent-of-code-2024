pub mod puzzle_01;
pub mod puzzle_02;

pub fn run() {
    println!("\nDay 01 Answers");
    println!("--------------");

    let file = "src/day_01/input_01";

    let answer1 = puzzle_01::distance_from_file(file);

    println!("Sum of scores: {}", answer1);

    let file = "src/day_01/input_01";

    let answer2 = puzzle_02::similarity_score_from_file(file);

    println!("Sum of scores: {}", answer2);
    println!("--------------");
}
