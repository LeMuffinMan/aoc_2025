
fn get_number(bank: &Vec<u32>, indexes: (usize, usize, usize)) -> u64 {
    let mut res: u64 = 0;
    let first = indexes.0;
    let second = indexes.1;
    let third = indexes.2;
    for (i, n) in bank.iter().enumerate() {
        if i == first || i == second || i == third {
            continue;
        }
        println!("Bank = {:?} | res = {res}", bank);
        res = res * 10;
        res += *n as u64;
    }
    res
}

fn get_max_joltage(bank: &Vec<u32>) -> u64 {
    let mut max = 0;
    for i in 0..bank.len() - 2 {
        for j in i + 1..bank.len() - 1{
            for h in j + 1..bank.len() {
                let n = get_number(bank, (i, j, h));
                if max < n {
                    max = n;
                    println!("New max = {max}");
                }
            }
        }
    }
    println!("Bank = {:?} | max = {max}", bank);
    max
}

pub fn part_2(input: &Vec<String>) -> u64 {
    let mut count = 0;
    println!("Banks = {:?}", input);
    // let input = vec!["987654321111111", "811111111111119", "234234234234278", "818181911112111"];
    for l in input {
        let mut bank = Vec::<u32>::new();
        for c in l.chars() {
            bank.push(c.to_digit(10).unwrap());
        }
        count += get_max_joltage(&bank);
    }
    count as u64
}

