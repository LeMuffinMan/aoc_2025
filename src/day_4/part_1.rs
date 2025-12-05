
fn get_adjacent_cells(map: &Vec<String>, x: usize, y: usize) -> Vec<char> {
    let mut vec = Vec::new();
    
    if x > 0 {
        let line = map[x - 1].as_bytes();
        vec.push(line[y] as char);
        if y > 0 {
            vec.push(line[y - 1] as char);
        }
        if y < line.len() - 1 {
            vec.push(line[y + 1] as char);
        }
    }
    if x < map.len() - 1 {
        let line = map[x + 1].as_bytes();
        vec.push(line[y] as char);
        if y > 0 {
            vec.push(line[y - 1] as char);
        }
        if y < line.len() - 1 {
            vec.push(line[y + 1] as char);
        }
    }
    let line = map[x].as_bytes();
    if y > 0 {
        vec.push(line[y - 1] as char);
    }
    if y < line.len() - 1 {
        vec.push(line[y + 1] as char)
    }
    vec
}

fn is_accessible_roll(map: &Vec<String>, x: usize, y: usize) -> usize {
    let mut roll_count = 0;
    let adjacent_cells = get_adjacent_cells(map, x, y);
    for cell in adjacent_cells {
        match cell {
            '@' => roll_count += 1,
            _ => continue,
        }
    }
    if roll_count < 4 {
        return 1;
    }
    return 0;
}

pub fn part_1(input: &mut Vec<String>) -> u64 {
    let mut count = 0;
    // let input: Vec<String> = vec![
    //     "..@@.@@@@.",
    //     "@@@.@.@.@@",
    //     "@@@@@.@.@@",
    //     "@.@@@@..@.",
    //     "@@.@@@@.@@",
    //     ".@@@@@@@.@",
    //     ".@.@.@.@@@",
    //     "@.@@@.@@@@",
    //     ".@@@@@@@@.",
    //     "@.@.@@@.@.",
    //     ]
    //     .into_iter()
    //     .map(|s| s.to_string())
    //     .collect(); 
    for i in 0..input.len() {
        let line = input[i].as_bytes();
        for j in 0..line.len() {
            match line[j] as char {
                '@' => count += is_accessible_roll(input, i, j),
                _ => continue,
            };
        }
        // println!("{:?}", input[i]);
    }
    // println!("Password = {count}");
    count as u64
}

