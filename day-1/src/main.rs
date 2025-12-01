use std::env;
use reqwest::blocking::Client;
use std::error::Error;

#[derive(Debug)]
struct Dial {
    value: i32,
    password: u32,
}

impl Dial {
    fn rotate(&mut self, dir: char, mut clicks: i32) {
        loop {
            match dir {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => unreachable!(),
            };
            clicks -= 1;
            if clicks == 0 { break };
        }
    }
    fn turn_left(&mut self) {
        if self.value == 0 {
            self.value = 99;
        } else {
            self.value -= 1;
            if self.value == 0 {
                self.password += 1;
            }
        }
    }
    fn turn_right(&mut self) {
        if self.value == 99 {
            self.value = 0;
            self.password += 1;
        } else {
            self.value += 1;
        }
    }
}

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

