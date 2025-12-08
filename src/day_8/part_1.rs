use crate::day_8::junction_boxe::JunctionBoxe;

fn get_junction_boxes(input: &Vec<String>) -> Vec<JunctionBoxe> {
    let mut junction_boxes: Vec<JunctionBoxe> = Vec::new();
    for line in input.into_iter() {
        let coords: Vec<f64> = line
            .split(',')
            .map(|coord| coord.parse::<f64>().unwrap())
            .collect();
        if let [x, y, z] = coords.as_slice() {
            junction_boxes.push(JunctionBoxe::new(*x, *y, *z));
        }
    }
    junction_boxes
}

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut junction_boxes = get_junction_boxes(&input);
    let mut last_dist: f64 = 0.0;
    for _ in 0..1000 {
        if let Some((b1, b2, dist)) = JunctionBoxe::get_closest(&junction_boxes, last_dist) {
            println!(
                "pair = ({:?} {:?}) ({:?} {:?}) dist = {dist}",
                junction_boxes[b1].x,
                junction_boxes[b1].y,
                junction_boxes[b2].x,
                junction_boxes[b2].y
            );
            last_dist = dist;
            match junction_boxes[b1].is_same_circuit(&junction_boxes[b2]) {
                true => continue,
                false => {
                    let mut last_circuit = JunctionBoxe::count_circuits(&junction_boxes);
                    JunctionBoxe::connect_boxes(&mut junction_boxes, b1, b2, &mut last_circuit);
                    junction_boxes.sort_by_key(|boxe| boxe.circuit);
                }
            }
        } else {
            unreachable!("No pair returned");
        }
    }
    JunctionBoxe::product_top_3_circuit_size(&mut junction_boxes) as u64
}
