use reqwest::blocking::Client;
use std::env;
use std::error::Error;

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
