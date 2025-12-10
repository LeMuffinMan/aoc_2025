
fn parse(input: &Vec<String>) -> (Vec<&str>, Vec<&str>) {
  let mut lights = Vec::new();
  let mut buttons = Vec::new();
  for line in input {
    if let Some((left, right)) = line.split_once(' ') {
      lights.push(left);
      if let Some((b, j)) = right.split_once('{') {
        buttons.push(b);
      } else {
        unreachable!("Not splittable");
      }
    } else {
      unreachable!("Not splittable");
    }
  }
  (lights, buttons)
}

fn get_presses(lights: &str, buttons: &str) -> u64 {
  let mut lights_target = Vec::new();
  let mut buttons_grid = Vec::new();
  let mut presses = 0;
  for c in lights.chars() {
    match c {
      '.' => lights_target.push(false),
      '#' => lights_target.push(true),
      _ => continue,
    }
  }
  buttons_grid = buttons.split_whitespace().collect();
  for l in lights_target {
    println!("lights_target = {:?}", l);
  }
  for b in buttons_grid {
    println!("buttons_grid = {:?}", b);
  }
  presses
}

pub fn part_1(input: &Vec<String>) -> u64 {
  let input = vec! [
"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
  ];
  let (lights, buttons) = parse(&input);
  let mut min = 0;
  for i in 0..lights.len() {
    println!("{:?} {:?}", lights[i], buttons[i]);
    let res = get_presses(lights[i], buttons[i]);
    if res < min {
      min = res;
    }
  }
  min
}
