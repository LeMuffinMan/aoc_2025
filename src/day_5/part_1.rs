
fn split_input(input : &mut Vec<String>) -> (Vec<String> , Vec<String>) {
  let mut ranges = Vec::new();
  let mut ingredients = Vec::new();

  while let Some(line) = input.pop() && !line.is_empty() {
    ingredients.push(line);
  }

  while let Some(line) = input.pop() {
    ranges.push(line);
  }
  (ranges, ingredients)
}

fn count_fresh_ingredients(ranges: Vec<String>, ing: String) -> u8 {
  for range in ranges {
    let (low, high) = range.split_once('-');
    if ing.len() > low && ing.len() < high {
      return 1;      
    } else if ing.len() == low.len() {
      //ici on compare par index
    } 
  }
  0
}

pub fn part_1(mut input: &mut Vec<String>) -> u64 {
  let mut count = 0;
  let (ranges, ingredients) = split_input(&mut input);
  for ing in ingredients {
    println!("#{count} {line}");
    count += count_fresh_ingredients(ranges, ing);
  }
  count
}
