fn get_worksheet(input: &Vec<String>) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut ops: Vec<char> = Vec::new();
    let mut worksheet: Vec<Vec<u64>> = Vec::new();
    for line in input {
        if line.find('+').is_some() || line.find('*').is_some() {
            ops.extend(line.chars().filter(|&c| c == '+' || c == '*'));
            break;
        }
        worksheet.push(
            line.split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect(),
        );
    }
    (worksheet, ops)
}

fn solve_worksheet(worksheet: Vec<Vec<u64>>, ops: Vec<char>) -> u64 {
    let mut res = 0;
    for (i, op) in ops.iter().enumerate() {
        let mut numbers = Vec::new();
        for row in &worksheet {
            numbers.push(row[i]);
        }
        res += match op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product::<u64>(),
            _ => unreachable!(),
        };
    }
    res
}

pub fn part_1(input: &Vec<String>) -> u64 {
    let (worksheet, ops) = get_worksheet(&input);
    solve_worksheet(worksheet, ops)
}
