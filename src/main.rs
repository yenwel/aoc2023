use std::env;

mod day_one;
mod utils;

fn main() {
    if let Some(day) = env::args().nth(1) {
        match day.as_str() {
            "one" => day_one::run_challenges(),
            &_ => println!("day not iplemented")
        }
    }
}
