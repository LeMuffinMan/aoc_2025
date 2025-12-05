use crate::day_4::part_1::part_1;
use crate::day_4::part_2::part_2;

pub fn solve_day_4(input: Vec<String>) {
    println!("Part 1 password = {:?}", part_1(&mut input.clone()));
    println!("Part 2 password = {:?}", part_2(&mut input.clone()));
}
