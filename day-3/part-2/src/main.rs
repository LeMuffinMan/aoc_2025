
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

fn get_max_index(vec: &Vec<u32>, start: usize, end: usize) -> usize {
    let mut max = 0;
    let mut max_i = 0;
    for i in start..end {
        if vec[i] > max {
            max = vec[i];
            max_i = i
        }
    }
    max_i
}

fn get_max_joltage(bank: &Vec<u32>) -> u64 {
    let mut res: u64 = 0;
    let mut digit = 11;
    let mut max = 0;
    for i in 0..bank.len() {
        max = get_max_index(&bank, max, bank.len() - digit);
        res += bank[max] as u64;
        if digit > 0 {
            res *= 10;
            max = i + 1;
            digit -= 1;
        } else {
            break;
        }
    }
    println!("Bank = {:?} | res = {res}", bank);
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    let input = get_input()?;
    let input = vec!["987654321111111", "811111111111119", "234234234234278", "818181911112111"];
    for l in input {
        let mut bank = Vec::<u32>::new();
        for c in l.chars() {
            bank.push(c.to_digit(10).unwrap());
        }
        count += get_max_joltage(&bank);
    }
    println!("password = {count}");
    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2025/day/3/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

