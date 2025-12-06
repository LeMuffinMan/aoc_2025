
fn get_fresh_IDs(ranges: Vec<String>) -> u64 {
  let mut count = 0;
  let mut max = 0;
  for range in ranges {
    let Some((low, high)) = range.split_once('-') else { unreachable!() };
    let low_n = low.parse::<u64>().unwrap();
    let high_n = high.parse::<u64>().unwrap();
    if low_n < max && max < high_n {
      count += high_n - max;
      max = high_n;
    } else if max < low_n {
      count += high_n - low_n + 1;
      max = high_n;
    } else if max == low_n {
      count += high_n - max;
      max = high_n;
    } else {
    }
  }
  count
}

pub fn part_2(input: &Vec<String>) -> u64 {
  let mut ranges = Vec::new();
  for line in input {
    if line.find('-') == None {
      break;
    }
    ranges.push(line.clone());
  }
  ranges.sort_by_key(|range| {
    let mut pair = range.split('-');
    pair.next().unwrap().parse::<u64>().unwrap()
  });
  // for l in &ranges {
  //   println!("{l}");
  // }
  get_fresh_IDs(ranges)
}
