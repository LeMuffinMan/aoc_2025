#[derive(PartialEq, Debug)]
pub struct JunctionBoxe {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub circuit: Option<usize>,
}

impl JunctionBoxe {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            circuit: None,
        }
    }
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    pub fn connect_boxes(
        boxes: &mut [Self],
        i: usize,
        j: usize,
        last_circuit: &mut usize,
    ) -> Option<f64> {
        let (b1, b2) = {
            let (left, right) = boxes.split_at_mut(j.max(i));
            if i < j {
                (&mut left[i], &mut right[0])
            } else {
                (&mut right[0], &mut left[j])
            }
        };
        let res = b1.x * b2.x;
        match (b1.get_circuit(), b2.get_circuit()) {
            (None, None) => {
                *last_circuit += 1;
                b1.set_circuit(Some(*last_circuit));
                b2.set_circuit(Some(*last_circuit));
            }
            (id, None) | (None, id) => {
                b1.set_circuit(id);
                b2.set_circuit(id);
            }
            (Some(id1), Some(id2)) => JunctionBoxe::connect_networks(boxes, id1, id2),
        };
        if JunctionBoxe::count_circuits(boxes) == 1 {
            Some(res)
        } else {
            None
        }
    }

    pub fn connect_networks(boxes: &mut [Self], id1: usize, id2: usize) {
        let (unified_circuit, circuit_to_delete) = match id1 < id2 {
            true => (id1, id2),
            false => (id2, id1),
        };
        // println!("unified = {unified_circuit} | to_delete = {circuit_to_delete}");
        for i in 0..boxes.len() {
            if let Some(id) = boxes[i].get_circuit() {
                if id == circuit_to_delete {
                    boxes[i].set_circuit(Some(unified_circuit));
                } else if id > circuit_to_delete {
                    boxes[i].set_circuit(Some(id - 1));
                }
            }
        }
    }

    pub fn is_same_circuit(&self, other: &Self) -> bool {
        if self.circuit == None || other.circuit == None {
            false
        } else {
            self.circuit == other.circuit
        }
    }

    pub fn get_circuit(&self) -> Option<usize> {
        self.circuit
    }

    pub fn set_circuit(&mut self, circuit: Option<usize>) {
        self.circuit = circuit;
    }

    pub fn count_circuits(boxes: &[Self]) -> usize {
        let mut count = 0;
        for i in 0..boxes.len() {
            match boxes[i].get_circuit() {
                None => count += 1,
                _ => continue,
            }
        }
        if let Some(boxe) = boxes.last() {
            match boxe.circuit {
                Some(id) => id + count,
                _ => 0,
            }
        } else {
            0
        }
    }

    pub fn remove_circuit(boxes: &mut [Self], id: usize) -> usize {
        let mut count = 0;
        if id != 0 {
            for i in 0..boxes.len() {
                if boxes[i].get_circuit() == Some(id) {
                    boxes[i].set_circuit(None);
                    count += 1;
                }
            }
        }
        count
    }

    pub fn get_largest_id(boxes: &[Self]) -> usize {
        let mut max = 0;
        let mut largest = 0;
        for i in 1..JunctionBoxe::count_circuits(boxes) {
            let size = JunctionBoxe::get_circuit_size(boxes, i);
            if max < size {
                max = size;
                largest = i;
            }
        }
        largest
    }

    pub fn product_top_3_circuit_size(boxes: &mut [Self]) -> usize {
        let mut res = 1;
        for _ in 0..3 {
            let size = JunctionBoxe::remove_circuit(boxes, Self::get_largest_id(boxes));
            // println!("{i}'s biggest circruit size = {size}");
            res *= size;
        }
        res
    }

    #[allow(dead_code)]
    pub fn print_circuits(boxes: &mut [Self]) {
        for i in 1..JunctionBoxe::count_circuits(boxes) {
            let size = JunctionBoxe::get_circuit_size(boxes, i);
            if size > 1 {
                println!("circuit #{i} | {:?} boxes", size);
            }
            for j in 0..boxes.len() {
                match boxes[j].get_circuit() {
                    None => {
                        if i == 0 {
                            println!("{:?}", boxes[j]);
                        }
                    }
                    Some(id) => {
                        if id == i {
                            println!("{:?}", boxes[j]);
                        }
                    }
                }
            }
        }
    }

    pub fn get_circuit_size(boxes: &[Self], id: usize) -> usize {
        let mut count = 0;
        for j in 0..boxes.len() {
            match boxes[j].get_circuit() {
                None => {
                    if id == 0 {
                        count += 1;
                    }
                }
                Some(boxe_id) => {
                    if boxe_id == id {
                        count += 1;
                    }
                }
            }
        }
        match count {
            0 => 1,
            _ => count,
        }
    }

    pub fn get_closest(boxes: &[Self], last_dist: f64) -> Option<(usize, usize, f64)> {
        let mut min = f64::MAX;
        let mut pair: Option<(usize, usize, f64)> = None;

        // println!("last dist = {last_dist}");
        for i in 0..boxes.len() {
            for j in 0..boxes.len() {
                if boxes[i] != boxes[j] {
                    let dist = boxes[i].distance(&boxes[j]);
                    if dist < min && dist > last_dist {
                        // println!("dist = {dist}");
                        min = dist;
                        pair = Some((i, j, dist));
                    }
                }
            }
        }
        pair
    }
}
