use crate::day_8::junction_boxe::JunctionBoxe;
use std::io::{self, Read};

fn get_junction_boxes(input: &Vec<String>) -> Vec<JunctionBoxe> {
    let mut junction_boxes: Vec<JunctionBoxe> = Vec::new();
    for (i, line) in input.into_iter().enumerate() {
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
    // let input: Vec<String> = vec![
    //     "162,817,812".into(),
    //     "57,618,57".into(),
    //     "906,360,560".into(),
    //     "592,479,940".into(),
    //     "352,342,300".into(),
    //     "466,668,158".into(),
    //     "542,29,236".into(),
    //     "431,825,988".into(),
    //     "739,650,466".into(),
    //     "52,470,668".into(),
    //     "216,146,977".into(),
    //     "819,987,18".into(),
    //     "117,168,530".into(),
    //     "805,96,715".into(),
    //     "346,949,466".into(),
    //     "970,615,88".into(),
    //     "941,993,340".into(),
    //     "862,61,35".into(),
    //     "984,92,344".into(),
    //     "425,690,689".into(),
    // ];
    let mut junction_boxes = get_junction_boxes(&input);
    let mut min: f64 = f64::MAX;
    let mut last_dist: f64 = 0.0;
    let len = junction_boxes.len();
    for i in 0..1000 {
        if let Some((b1, b2, dist)) = JunctionBoxe::get_closest(&junction_boxes, last_dist) {
            println!("pair = {:?} {:?} dist = {dist}", junction_boxes[b1], junction_boxes[b2]);
            last_dist = dist;
            match junction_boxes[b1].is_same_circuit(&junction_boxes[b2]) {
                true => continue,
                false | _ => {
                    let mut last_circuit = JunctionBoxe::count_circuits(&junction_boxes);
                    JunctionBoxe::connect_boxes(&mut junction_boxes, b1, b2, &mut last_circuit);
                    junction_boxes.sort_by_key(|boxe| boxe.circuit);
                },
            }
        } else {
            unreachable!("No pair returned");
        }
        // let len = JunctionBoxe::count_circuits(&mut junction_boxes);
        // println!("circuits count = {}", len);
        // JunctionBoxe::print_circuits(&mut junction_boxes);
        // println!("");
        // std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    junction_boxes.sort_by_key(|boxe| boxe.circuit);
    for b in &junction_boxes {
        // println!("{:?}", b);
    }
    let len = JunctionBoxe::count_circuits(&mut junction_boxes);
    // println!("circuits count = {}", len);
    JunctionBoxe::print_circuits(&mut junction_boxes);
    JunctionBoxe::product_top_3_circuit_size(&mut junction_boxes) as u64
}
