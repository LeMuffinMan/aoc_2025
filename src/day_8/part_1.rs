use crate::day_8::junction_boxe::JunctionBoxe;
use crate::day_8::circuit::Circuit;



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

fn init_circuits(input: &Vec<String>) -> Vec<Circuit> {
    let circuits = Vec::new();
    for line in input {
        circuits.push(Circuit {
                parent: None,
                size: 1
        });
    }
    circuits
} 

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut circuits = circuit::init_circuits(&input);
    circuits.iter().map(|c| println!("{:?}", c));

    // let mut boxes = get_junction_boxes(&input);
    // let mut last_dist: f64 = 0.0;
    // for _ in 0..1000 {
    //     if let Some((b1, b2, dist)) = JunctionBoxe::get_closest(&boxes, last_dist) {
    //         println!(
    //             "pair = ({:?} {:?}) ({:?} {:?}) dist = {dist}",
    //             boxes[b1].x,
    //             boxes[b1].y,
    //             boxes[b2].x,
    //             boxes[b2].y
    //         );
    //         last_dist = dist;
    //         match boxes[b1].is_same_circuit(&boxes[b2]) {
    //             true => continue,
    //             false => {
    //                 let mut last_circuit = JunctionBoxe::count_circuits(&boxes);
    //                 JunctionBoxe::connect_boxes(&mut boxes, b1, b2, &mut last_circuit);
    //                 boxes.sort_by_key(|boxe| boxe.circuit);
    //             }
    //         }
    //     } else {
    //         unreachable!("No pair returned");
    //     }
    // }
    // JunctionBoxe::product_top_3_circuit_size(&mut boxes) as u64
}
