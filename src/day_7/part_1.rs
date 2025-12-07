fn draw_beams(map: &mut Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    let mut count = 0;
    if x > map.len() - 1 {
        return 0;
    }
    if y > map[x].len() {
        return 0;
    }
    match map[x][y] {
        '.' => {
            map[x][y] = '|';
            count += draw_beams(map, x + 1, y);
        }
        '^' => {
            count += 1;
            count += draw_beams(map, x, y - 1);
            count += draw_beams(map, x, y + 1);
        }
        _ => {}
    }
    count
}

// fn print_map(map: &Vec<Vec<char>>) {
//   map.clone().into_iter()
//       .map(|l| l.into_iter().collect::<String>())
//       .for_each(|s| println!("{}", s));
// }

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut count = 0;
    let mut map = input
        .into_iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == 'S' {
                count += draw_beams(&mut map, x + 1, y);
            }
        }
    }
    count
}
