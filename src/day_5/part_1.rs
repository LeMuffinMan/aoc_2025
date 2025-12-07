fn split_input(input: &mut Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    while let Some(line) = input.pop()
        && !line.trim().is_empty()
    {
        ingredients.push(line);
    }

    while let Some(line) = input.pop() {
        ranges.push(line);
    }
    (ranges, ingredients)
}

fn is_fresh_ingredients(ranges: &Vec<String>, ing: &String) -> bool {
    for range in ranges {
        let Some((low, high)) = range.split_once('-') else {
            unreachable!()
        };
        if ing.parse::<u64>().unwrap() >= low.parse::<u64>().unwrap()
            && ing.parse::<u64>().unwrap() <= high.parse::<u64>().unwrap()
        {
            return true;
        }
    }
    return false;
}

pub fn part_1(mut input: &mut Vec<String>) -> u64 {
    let mut count = 0;
    let (ranges, ingredients) = split_input(&mut input);
    // for range in &ranges {
    //   println!("Range = {range}");
    // }
    for ing in ingredients {
        // println!("Ing = {ing}");
        if is_fresh_ingredients(&ranges, &ing) {
            count += 1;
        }
    }
    // println!("count = {count}");
    count as u64
}
