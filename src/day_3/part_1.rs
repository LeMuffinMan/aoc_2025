
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

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut count = 0;
    // let input = vec!["987654321111111", "811111111111119", "234234234234278", "818181911112111"];
    for l in input {
        let mut res = 0;
        let mut bank = Vec::<u32>::new();
        for c in l.chars() {
            bank.push(c.to_digit(10).unwrap());
        }
        let max = get_max_index(&bank, 0, bank.len() - 1);
        res += bank[max];
        res *= 10;
        let max = get_max_index(&bank, max + 1, bank.len());
        res += bank[max];
        count += res;
    }
    count as u64
}

