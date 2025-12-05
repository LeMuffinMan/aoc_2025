#!/bin/bash

YEAR=$(date "+%Y")
MOUNTH=$(date "+%m")
DAY=$(date "+%d")
HOUR=$(date "+%H")

generate_day() {
  mkdir src/day_$DAY
  echo "
mod part_1;
mod part_2;

use part_1::part_1;
use part_2::part_2;

pub fn solve_day_$DAY(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>> {
    println!("Part 1 password = {:?}", part_1(&input));
    // println!("Part 2 password = {:?}", part_2(&input));
}
  " >> src/day_${DAY}/day_${DAY}.rs

  echo "
  pub mod day_$DAY;

  pub use day_$DAY::solve_day_$DAY;
  " >> src/day_$DAY/mod.rs

  echo "
fn part_1(input: Vec<String>) -> u64 {
  let mut count = 0;
  for line in input {
    println!(\"#{count} {line}\");
    count += 1;
  }
  count
}" >> src/day_$DAY/part_1.rs

  echo "
fn part_2(input: Vec<String>) -> u64 {
  let mut count = 0;
  for line in input {
    println!(\"#{count} {line}\");
    count += 1;
  }
  count
}" >> src/day_$DAY/part_2.rs


  sed -i "s/_ => {/\"$DAY\" => solve_day_$DAY(input),\n        &/" src/input.rs
  sed -i "s/use crate::day_1::solve_day_1;/use crate::day_$DAY::solve_day_$DAY;\n&/" src/input.rs
  sed -i "s/mod input;/&\nmod day_$DAY;/" src/main.rs
}

if [ -z "$1" ]; then 
  if [[ $YEAR -eq 2025 && $DAY -lt 13 ]]; then
    if [ $HOUR -lt 6 ]; then
      DAY=$(($DAY - 1))
    fi
    if [ ! -d "src/day_$DAY" ]; then
      generate_day
    fi
  else
    echo "Usage: ./aoc <day>"
    exit 1
  fi
elif ! expr "$1" + 0 >/dev/null 2>&1; then
  echo "Usage: ./aoc <day>"
  exit 1
elif [[ "$DAY" -gt 12 ]]; then
  echo "12 puzzles available in 2025"
  exit 1
else
  DAY=$1
fi

cargo run -- $DAY

