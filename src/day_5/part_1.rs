
fn split_input(input : &mut Vec<String>) -> (Vec<String> , Vec<String>) {
  let mut ranges = Vec::new();
  let mut ingredients = Vec::new();

  while let Some(line) = input.pop() && !line.trim().is_empty() {
    ingredients.push(line);
  }

  while let Some(line) = input.pop() {
    ranges.push(line);
  }
  (ranges, ingredients)
}

fn count_fresh_ingredients(ranges: &Vec<String>, ing: String) -> bool {
  for range in ranges {
    let Some((low, high)) = range.split_once('-') else { println!("range [{range}] not splitable on -"); return false };
    let ing_len = ing.len();
    let low_len = low.len();
    let high_len = high.len();
    if ing_len < low_len || ing_len < high_len {
      return false;
    }
    if ing_len == low_len {
      return *ing < *low;
    }
    else if ing_len == high_len {
      return *ing > *low
    }
    println!("unreachable");
  }
  false
}

pub fn part_1(mut input: &mut Vec<String>) -> u64 {
  let mut input = vec![
      "3-5".to_string(),
      "10-14".to_string(),
      "16-20".to_string(),
      "12-18".to_string(),
      "\n".to_string(),
      "1".to_string(),
      "5".to_string(),
      "8".to_string(),
      "11".to_string(),
      "17".to_string(),
      "32".to_string(),
  ];
  let mut count = 0;
  let (ranges, ingredients) = split_input(&mut input);
  for range in &ranges {
    // println!("Range = {range}");
  }
  for ing in ingredients {
    // println!("Ing = {ing}");
    if count_fresh_ingredients(&ranges, ing) {
      count += 1;
    };
  }
  println!("count = {count}");
  count as u64
}
