use std::{fs, path::Path};

pub fn read_to_string(file: &str) -> String {
    let path = Path::new(&file);
    let content = fs::read_to_string(path);
    match content {
        Ok(c) => return c,
        Err(e) => panic!("unable to read file: {}, err: {}", file, e),
    }
}

pub fn run_day(day: u8, puzzle_01: &dyn Fn(&str) -> i32, puzzle_02: &dyn Fn(&str) -> i32) {
    println!("\nDay {:02} Answers", day);
    println!("--------------");

    let file = format!("src/day_{:02}/input_01", day);

    let answer1 = puzzle_01(&file);
    println!("Puzzle 01: {}", answer1);

    let answer2 = puzzle_02(&file);
    println!("Puzzle 02: {}", answer2);

    println!("--------------");
}
