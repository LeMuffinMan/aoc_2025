use std::collections::HashMap;

fn draw_beams(
    map: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    hashmap: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let mut count = 0;
    if x > map.len() - 1 {
        return 1;
    }
    if y >= map[x].len() {
        return 1;
    }
    if let Some(&res) = hashmap.get(&(x, y)) {
        return res;
    }
    match map[x][y] {
        '.' => {
            count += draw_beams(map, x + 1, y, hashmap);
        }
        '^' => {
            if map[x][y - 1] != '^' {
                count += draw_beams(map, x, y - 1, hashmap);
            }
            if map[x][y + 1] != '^' {
                count += draw_beams(map, x, y + 1, hashmap);
            }
        }
        _ => {}
    }
    hashmap.insert((x, y), count);
    count
}

// fn print_map(map: &Vec<Vec<char>>) {
//   // let mut count = 0;
//
//   map.clone().into_iter()
//       .map(|l| {
//       // println!("{count}");
//       // count += 1;
//       l.into_iter().collect::<String>()
//     })
//       .for_each(|s| println!("{}", s));
// }

pub fn part_2(input: &Vec<String>) -> u64 {
    let mut count = 0;
    let mut hashmap = HashMap::new();
    let map = input
        .into_iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == 'S' {
                count += draw_beams(&map, x + 1, y, &mut hashmap);
            }
        }
    }
    count
}
