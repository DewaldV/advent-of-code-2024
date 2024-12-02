mod day_01;
mod day_02;
mod util;

pub use self::util::read_to_string;

fn main() {
    day_01::run();
    day_02::run();
}
