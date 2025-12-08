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

pub fn part_2(input: &Vec<String>) -> u64 {
    let mut boxes = get_junction_boxes(&input);
    let mut last_dist: f64 = 0.0;
    loop {
        let circuits_count = JunctionBoxe::count_circuits(&boxes);
        if let Some((b1, b2, dist)) = JunctionBoxe::get_closest(&boxes, last_dist) {
            println!(
                "circuit count = {:} pair = ({:?} {:?}) ({:?} {:?}) dist = {dist}",
                circuits_count, boxes[b1].x, boxes[b1].y, boxes[b2].x, boxes[b2].y
            );
            last_dist = dist;
            match boxes[b1].is_same_circuit(&boxes[b2]) {
                false => {
                    let mut last_circuit = JunctionBoxe::count_circuits(&boxes);
                    if let Some(res) =
                        JunctionBoxe::connect_boxes(&mut boxes, b1, b2, &mut last_circuit)
                    {
                        return res as u64;
                    }
                    boxes.sort_by_key(|boxe| boxe.circuit);
                }
                true => continue,
            }
        } else {
            unreachable!("No pair returned");
        }
    }
}
