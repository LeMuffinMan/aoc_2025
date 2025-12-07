use crate::day_1::solve_day_1;
use crate::day_2::solve_day_2;
use crate::day_3::solve_day_3;
use crate::day_4::solve_day_4;
use crate::day_5::solve_day_5;
use crate::day_6::solve_day_6;
use crate::day_7::solve_day_7;
use crate::input::get_input;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let day = std::env::args().nth(1).expect("Usage: cargo run -- <day>");
    let input = get_input(&day)?;
    match day.as_str() {
        "1" => solve_day_1(input),
        "2" => solve_day_2(input),
        "3" => solve_day_3(input),
        "4" => solve_day_4(input),
        "5" => solve_day_5(input),
        "6" => solve_day_6(input),
        "7" => solve_day_7(input),
        _ => {
            println!("Day {} not implemented", day);
        }
    };
    Ok(())
}
