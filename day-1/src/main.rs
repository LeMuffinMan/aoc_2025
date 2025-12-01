use std::env;
use reqwest::blocking::Client;
use std::error::Error;

mod dial;
use dial::Dial;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let mut dial = Dial { value: 50, password:0 };

    for rot in &input {
        let (dir_char, clicks) = rot.split_at(1);
        let dir = dir_char.parse().unwrap();
        let num: i32 = clicks.parse().unwrap();
        dial.rotate(dir, num);
        // println!("{:?} | {:?}", dial, rot);
    }
    println!("password = {:?}", dial.password);
    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2025/day/1/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

