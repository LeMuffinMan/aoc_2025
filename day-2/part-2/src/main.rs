
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

fn seek_sequence(str: &String, size: usize) -> u64 { 
    let len = str.len();
    let chunk = len / size;
    let vec: Vec<char> = str.chars().collect();
    let seq = vec
        .chunks(chunk)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();
    
    if seq.first().map(|f| seq.iter().all(|n| n == f)).unwrap() {
        println!("{str} Invalid: ({:?} {size})", seq);
        return str.parse().unwrap();
    } 

    return 0;
}

fn find_invalids_ids(range: &str) -> u64 {
    let mut ret = 0;
    if let Some((low, high)) = range.split_once('-') {
        let num_low: u64 = low.parse().unwrap();
        let num_high: u64 = high.parse().unwrap();
        for n in num_low..=num_high {
            let n_str = n.to_string();
            let mut size = 2;
            while size <= n_str.len() {
                let res = seek_sequence(&n_str, size);
                if res > 0 {
                    ret += res;
                    break;
                }
                size += 1;
            }
        }
    } 
    ret
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let mut count = 0;
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let sequences = input.split(','); 
    for range in sequences {
        println!("\nTesting range : {range}");
        count += find_invalids_ids(range);
    }
    println!("passsword = {count}");
    Ok(())
}

fn get_input() -> Result<String, Box<dyn Error>> {
    dotenv::from_path("../../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2025/day/2/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let line = response.trim().to_string();
    Ok(line)
}

