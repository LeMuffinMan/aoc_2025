use crate::day_4::solve_day_4;
use crate::day_3::solve_day_3;
use crate::day_2::solve_day_2;
use crate::day_1::solve_day_1;
use std::error::Error;
use std::env;
use reqwest::blocking::Client;

pub fn solve(day: String) -> Result<u32, Box<dyn std::error::Error>> {
    let input = get_input(&day)?;
    let password = match day.as_str() {
        "1" => solve_day_1(input),
        "2" => solve_day_2(input),
        "3" => solve_day_3(input),
        "4" => solve_day_4(input),
        _ => {
            println!("Day {} not implemented", day);
            Ok(0)
        }
    }; 
    password
}

pub fn get_input(day: &String) -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path(".env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = format!("https://adventofcode.com/2025/day/{}/input", day);

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}
