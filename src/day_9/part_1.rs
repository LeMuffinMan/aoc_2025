

pub fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

pub fn get_area(p1:  (f64, f64), p2: (f64, f64)) -> f64 {
  let dx = (p1.0 - p2.0).abs() + 1.0;
  let dy = (p1.1 - p2.1).abs() + 1.0;
  dx * dy
}

pub fn part_1(input: &Vec<String>) -> u64 {
  let mut tiles: Vec<(f64, f64)> = Vec::new();
  for line in input {
    let coord: Vec<f64> = line.split(',').map(|coord| coord.parse::<f64>().unwrap())
      .collect();
    let point = (coord[0], coord[1]);
    tiles.push(point);
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
