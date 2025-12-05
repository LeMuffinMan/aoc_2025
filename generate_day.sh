#!/bin/sh

generate_day() {

  if [ -z "$DAY" ]; then 
    echo "Usage ./generate_day.sh <day>"
  fi

  mkdir src/day_$DAY
  echo "
use crate::day_$DAY::part_1::part_1;
#[allow(unused_imports)]
use crate::day_$DAY::part_2::part_2;

pub fn solve_day_$DAY(input: Vec<String>)  {
    println!(\"Part 1 password = {:?}\", part_1(&input));
    // println!(\"Part 2 password = {:?}\", part_2(&input));
}
  " >> src/day_${DAY}/day_${DAY}.rs

  echo "
pub mod day_$DAY;
pub mod part_1;
pub mod part_2;

pub use day_$DAY::solve_day_$DAY;
  " >> src/day_$DAY/mod.rs

  echo "
pub fn part_1(input: &Vec<String>) -> u64 {
  let mut count = 0;
  for line in input {
    println!(\"#{count} {line}\");
    count += 1;
  }
  count
}" >> src/day_$DAY/part_1.rs

  echo "
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn part_2(input: &Vec<String>) -> u64 {
  let mut count = 0;
  for line in input {
    // println!(\"#{count} {line}\");
    count += 1;
  }
  count
}" >> src/day_$DAY/part_2.rs


  sed -i "s/_ => {/\"$DAY\" => solve_day_$DAY(input),\n        &/" src/main.rs
  sed -i "s/use crate::input::get_input;/&\nuse crate::day_$DAY::solve_day_$DAY;/" src/main.rs
  sed -i "s/mod input;/&\nmod day_$DAY;/" src/main.rs
}
