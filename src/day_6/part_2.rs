fn get_column(map: &Vec<Vec<char>>, y: usize) -> (Vec<char>, Option<char>) {
    let mut number = Vec::new();
    for x in 0..map.len() {
        match map[x][y] {
            '*' | '+' => return (number, Some(map[x][y])),
            ' ' => continue,
            _ => number.push(map[x][y]),
        }
    }
    (number, None)
}

fn get_problem(map: &Vec<Vec<char>>, y: &mut usize) -> (Vec<Vec<char>>, char) {
    let mut digits: Vec<Vec<char>> = Vec::new();
    loop {
        let (number, op) = get_column(&map, *y);
        digits.push(number);
        match op {
            Some(op) => return (digits, op),
            _ => *y -= 1,
        }
    }
}

fn parse(digits: Vec<char>) -> u64 {
    digits
        .into_iter()
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn product(problem: Vec<Vec<char>>) -> u64 {
    problem.into_iter().map(|digits| parse(digits)).product()
}

fn sum(problem: Vec<Vec<char>>) -> u64 {
    problem.into_iter().map(|digits| parse(digits)).sum()
}

pub fn part_2(input: &Vec<String>) -> u64 {
    let map: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
    let mut res = 0;
    let mut y = map[0].len() - 1;
    loop {
        let (problem, op) = get_problem(&map, &mut y);
        res += match op {
            '*' => product(problem),
            '+' => sum(problem),
            _ => unreachable!("Unexpected operator"),
        };
        match y == 0 {
            true => break,
            false => y -= 2,
        };
    }
    res
}
