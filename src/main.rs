mod input;
mod day_4;
mod day_3;
mod day_2;
mod day_1;
use crate::input::solve;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let day = std::env::args().nth(1).expect("Usage: cargo run -- <day>");
    solve(day).unwrap();
    Ok(())
}
