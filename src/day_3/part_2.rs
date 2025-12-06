
fn get_max_joltage(bank: &String) -> u64 {
    let digits: Vec<char> = bank.chars().collect();
    let k = 12;
    let mut stack: Vec<char> = Vec::new();
    let mut to_remove = digits.len() - k;

    for &digit in &digits {
        while !stack.is_empty() && stack.last().unwrap() < &digit && to_remove > 0 {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(digit);
    }

    let res: String = stack.into_iter().take(k).collect();
    res.parse().unwrap()
}

pub fn part_2(input: &Vec<String>) -> u64 {
    input.iter().map(|bank| get_max_joltage(bank)).sum()
}

