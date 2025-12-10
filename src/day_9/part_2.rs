

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

fn get_area(p1:  (f64, f64), p2: (f64, f64)) -> f64 {
  let dx = (p1.0 - p2.0).abs() + 1.0;
  let dy = (p1.1 - p2.1).abs() + 1.0;
  dx * dy
}

fn get_minmax(tiles: &Vec<(f64, f64)>) -> (usize, usize, usize, usize){
  let mut max_x = 0.0;
  let mut max_y = 0.0;
  let mut min_x = <f64>::MAX;
  let mut min_y = <f64>::MAX;
  for t in tiles {
    if t.0 > max_x {
      max_x = t.0;
    } else if t.0 < min_x {
      min_x = t.0;
    }
    if t.1 > max_y {
      max_y = t.1;
    } else if t.1 < min_y {
      min_y = t.1;
    }
  }
  (max_x as usize, max_y as usize, min_x as usize, min_y as usize)
}

fn print_dot_map(map: &Vec<Vec<char>>) {
  for l in map {
    println!("{:?}", l);
  }
  println!("=============================");
}

fn get_next_tile(t1: (f64, f64), t2: (f64, f64), dot_map: &mut Vec<Vec<char>>) {
  let start = t1;
  let stop = t2;
  if start.0 == stop.0 {
    //move on y
    if stop.1 - start.1 > 0.0 {
      for i in start.1 as usize..stop.1 as usize {
        if stop.1 - start.1 > 0.0 {
          print_dot_map(&dot_map);
          println!("stop.1 = {} | start.1 = {}", stop.1, start.1);
          dot_map[start.0 as usize][i] = 'X';
        }
      }
    }
  } else {
    //move on x
  }
}

fn get_dot_map(tiles: &Vec<(f64, f64)>) -> Vec<Vec<char>> {
  let (max_x, max_y, min_x, min_y) = get_minmax(tiles);
  let width = (max_x + 1 - min_x - 1) as usize;
  let height = (max_y + 1 - min_y - 1) as usize;
  println!("width = {width} | height = {height}");
  let mut dot_map = vec![vec!['.'; height + 1]; width + 1];
  for x in min_x..max_x {
    for y in min_y..max_y {
      for t in tiles {
        if x == t.0 as usize && y == t.1 as usize {
          dot_map[x][y] = '#';
        }
      }
    }
  }
  print_dot_map(&dot_map);
  for i in 1..tiles.len() {
    get_next_tile(tiles[i - 1], tiles[i], &mut dot_map);
  }
  for l in &dot_map {
    println!("{:?}", l);
  }
  dot_map
}

pub fn part_2(input: &Vec<String>) -> u64 {
  // let input = vec![
  //   "7,1".to_string(),
  //   "11,1".to_string(),
  //   "11,7".to_string(),
  //   "9,7".to_string(),
  //   "9,5".to_string(),
  //   "2,5".to_string(),
  //   "2,3".to_string(),
  //   "7,3".to_string(),
  // ];
  for l in input {
    println!("{:?}", l);
  }
  // return 42;
  let mut tiles: Vec<(f64, f64)> = Vec::new();
  for line in input {
    let coord: Vec<f64> = line.split(',').map(|coord| coord.parse::<f64>().unwrap())
      .collect();
    let point = (coord[0], coord[1]);
    tiles.push(point);
  }
  let dot_map = get_dot_map(&tiles);
  for line in dot_map {
    println!("{:?}", line);
  }
  let mut max = 0.0;
  let mut res: Option<(usize, usize)> = None;
  for i in 0..tiles.len() {
    for j in 0..tiles.len() {
      if j == i {
        continue;
      }
      let dist: f64 = distance(tiles[i], tiles[j]);
      if dist > max {
        max = dist;
        res = Some((i, j));
      }
    }
  }
  if let Some((i, j)) = res {
    get_area(tiles[i], tiles[j]) as u64
  } else {
    unreachable!("No max distance found")
  }
}
