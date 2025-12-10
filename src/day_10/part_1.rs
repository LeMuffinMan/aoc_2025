
pub fn part_1(input: &Vec<String>) -> u64 {
  let mut count = 0;
  for line in input {
    println!("#{count} {line}");
    count += 1;
  }
  count
}
