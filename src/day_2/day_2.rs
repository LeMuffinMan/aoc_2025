use crate::day_2::part_1;
use crate::day_2::part_2;

pub fn solve_day_2(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>> {
    println!("Part 1 password = {:?}", part_1::solve_part_1(&input));
    println!("Part 2 password = {:?}", part_2::solve_part_2(&input));
    Ok(0)
}
