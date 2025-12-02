
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

fn find_invalids_ids(range: &str, count: &mut u64) -> u64 {
    if let Some((low, high)) = range.split_once('-') {
        println!("splitted = {low} {high}");
        let mut num_low: u64 = low.parse().unwrap();
        let mut num_high: u64 = high.parse().unwrap();
        // if num_low > num_high {
        //     let mut tmp = num_low;
        //     num_low = num_high;
        //     num_high = tmp;
        // }
        for ref mut n in num_low..=num_high {
            let n_str = n.to_string();
            // print!("n_str = {n_str} : ");
            if n_str.len() % 2 == 0 {
                let (left, right) = n_str.split_at(n_str.len() / 2);  
                if left == right {
                    println!("INVALID : {n}");
                    *count += *n;
                } else {
                    println!("Valid : {n}");
                }
            } else {
                println!("Valid : {n}");
            }
            *n += 1;
        }
    }
    *count
}
//19562842130 -> too low
//19562842152 -> too low
//
//20045852680 -> going down -> too high

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let mut count = 0;
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let sequences = input.split(','); 
    for range in sequences {
        // println!("{:?}", range);
        count = find_invalids_ids(range, &mut count);
    }
    println!("passsword = {count}");
    Ok(())
}

fn get_input() -> Result<String, Box<dyn Error>> {
    dotenv::from_path("../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2025/day/2/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let line = response[0..response.len() - 2].to_string();
    Ok(line)
}

