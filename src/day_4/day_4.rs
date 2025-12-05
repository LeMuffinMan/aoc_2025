use crate::day_4::part_1::part_1;
use crate::day_4::part_2::part_2;

pub fn solve_day_4(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>> {
    println!("Part 1 password = {:?}", part_1(&mut input.clone()));
    println!("Part 2 password = {:?}", part_2(&mut input.clone()));
    Ok(0)
}
