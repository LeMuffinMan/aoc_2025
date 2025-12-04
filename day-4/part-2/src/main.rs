
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

fn get_adjacent_cells(map: &Vec<String>, x: usize, y: usize) -> Vec<char> {
    let mut vec = Vec::new();
    
    if x > 0 {
        let line = map[x - 1].as_bytes();
        vec.push(line[y] as char);
        if y > 0 {
            vec.push(line[y - 1] as char);
        }
        if y < line.len() - 1 {
            vec.push(line[y + 1] as char);
        }
    }
    if x < map.len() - 1 {
        let line = map[x + 1].as_bytes();
        vec.push(line[y] as char);
        if y > 0 {
            vec.push(line[y - 1] as char);
        }
        if y < line.len() - 1 {
            vec.push(line[y + 1] as char);
        }
    }
    let line = map[x].as_bytes();
    if y > 0 {
        vec.push(line[y - 1] as char);
    }
    if y < line.len() - 1 {
        vec.push(line[y + 1] as char)
    }
    vec
}

fn is_accessible_roll(map: &Vec<String>, x: usize, y: usize) -> Option<(usize, usize)> {
    let mut roll_count = 0;
    let adjacent_cells = get_adjacent_cells(map, x, y);
    for cell in adjacent_cells {
        match cell {
            '@' => roll_count += 1,
            _ => continue,
        }
    }
    if roll_count < 4 {
        return Some((x, y));
    }
    return None;
}

fn remove_rolls(map: &mut Vec<String>, cells: & mut Vec<(usize, usize)>) -> usize {
    let mut count = 0;
    for (x, y) in cells {
        unsafe {
            let line = map[*x].as_bytes_mut();
            line[*y] = b'x';
            count += 1;
        }
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    let mut removable = Vec::new();
    let mut input = get_input()?;
    // let mut input: Vec<String> = vec![
    //     "..@@.@@@@.",
    //     "@@@.@.@.@@",
    //     "@@@@@.@.@@",
    //     "@.@@@@..@.",
    //     "@@.@@@@.@@",
    //     ".@@@@@@@.@",
    //     ".@.@.@.@@@",
    //     "@.@@@.@@@@",
    //     ".@@@@@@@@.",
    //     "@.@.@@@.@.",
    //     ]
    //     .into_iter()
    //     .map(|s| s.to_string())
    //     .collect(); 

    loop {
        for i in 0..input.len() {
            let line= input[i].as_bytes();
            for j in 0..line.len() {
                match line[j] as char {
                    '@' => {
                        if let Some(cells) = is_accessible_roll(&input, i, j) {
                            removable.push(cells);
                        }
                    },
                    _ => continue,
                };
            }
        }
        if removable.is_empty() {
            break;
        }
        count += remove_rolls(&mut input, &mut removable);
        removable.clear();
    }
    println!("Password = {count}");

    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2025/day/4/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

