
use crate::day_5::part_1::part_1;
#[allow(unused_imports)]
use crate::day_5::part_2::part_2;

pub fn solve_day_5(input: Vec<String>)  {
    println!("Part 1 password = {:?}", part_1(&mut input.clone()));
    println!("Part 2 password = {:?}", part_2(&input));
}
  
