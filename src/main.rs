mod day_01;
mod util;

pub use self::util::read_to_string;

fn main() {
    day01();
}

fn day01() {
    let file = "src/day_01/input_01";

    let answer1 = day_01::puzzle_01::distance_from_file(file);

    println!("Sum of scores: {}", answer1);

    let file = "src/day_01/input_01";

    let answer2 = day_01::puzzle_02::similarity_score_from_file(file);

    println!("Sum of scores: {}", answer2);
}
