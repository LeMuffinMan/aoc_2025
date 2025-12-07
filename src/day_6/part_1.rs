

pub fn part_1(input: &Vec<String>) -> u64 {
  let mut worksheet : Vec<Vec<u64>> = Vec::new(); 
  let mut ops : Vec<char> = Vec::new();
  for line in input {
    if line.find('+').is_some() || line.find('*').is_some() {
      ops.extend(line.chars().filter(|&c| c == '+' || c == '*'));
      break;
    }
    worksheet.push(line.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect());
  }
  let mut count = 0;
  for (i, op) in ops.iter().enumerate() {
    let mut numbers = Vec::new();
    for row in &worksheet {
      numbers.push(row[i]);
    }
    let res = match op {
      '+' => numbers.iter().sum(),
      '*' => numbers.iter().product::<u64>(), 
      _ => unreachable!(),
    };
    count += res as usize;
  }
  count as u64
}
