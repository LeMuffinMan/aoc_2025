
fn find_invalids_ids(range: &str, count: &mut u64) -> u64 {
    if let Some((low, high)) = range.split_once('-') {
        println!("splitted = {low} {high}");
        let num_low: u64 = low.parse().unwrap();
        let num_high: u64 = high.parse().unwrap();
        for n in num_low..=num_high {
            let n_str = n.to_string();
            if n_str.len() % 2 == 0 {
                let (left, right) = n_str.split_at(n_str.len() / 2);  
                if left == right {
                    println!("INVALID : {n}");
                    *count += n;
                } else {
                    println!("Valid : {n}");
                }
            } else {
                println!("Valid : {n}");
            }
        }
    } 
    *count
}

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut count = 0;
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    for line in input {
        for range in line.split(',') {
            println!("\nTesting range : {range}");
            count += find_invalids_ids(range, &mut count);
        }
    }
    println!("passsword = {count}");
    count
}

