
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

fn is_accessible_roll(map: &Vec<String>, x: usize, y: usize) -> Option<(usize, usize)> {
    let mut roll_count = 0;
    let adjacent_cells = get_adjacent_cells(map, x, y);
    for cell in adjacent_cells {
        match cell {
            '@' => roll_count += 1,
            _ => continue,
        }
    }
    if roll_count < 4 {
        return Some((x, y));
    }
    return None;
}

fn remove_rolls(map: & Vec<String>, cells: & mut Vec<(usize, usize)>) -> (usize, Vec<String>) {
    let mut count = 0;
    let mut new_map = Vec::new();
    for (i, line) in map.iter().enumerate() {
        let mut new_line = String::new();
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '@' => {
                    if cells.contains(&(i, j)) {
                        new_line.push('x');
                        count += 1;
                    } else {
                        new_line.push('@');
                    }
                },
                _ => new_line.push(ch),
            }
        }
        new_map.push(new_line);
    }
    (count, new_map)
}

pub fn part_2(input: &mut Vec<String>) -> u64 {
    let mut count = 0;
    let mut removable = Vec::new();
    // let mut input: Vec<String> = vec![
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

    loop {
        for i in 0..input.len() {
            let line= input[i].as_bytes();
            for j in 0..line.len() {
                match line[j] as char {
                    '@' => {
                        if let Some(cells) = is_accessible_roll(&input, i, j) {
                            removable.push(cells);
                        }
                    },
                    _ => continue,
                };
            }
        }
        if removable.is_empty() {
            break;
        }
        let (removed, new_map) = remove_rolls(input, &mut removable);
        *input = new_map;
        count += removed;
        removable.clear();
    }
    count as u64
}

