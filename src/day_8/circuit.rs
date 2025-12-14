
pub struct Circuit {
    parent: Option<usize>,
    size: u64,
}

impl Circuit {
    pub fn join_circuits(&mut self, other: Circuit, circuits: Vec<Circuit>) {
        match (self.parent, other.parent) {
            (None, None) => {
                other.parent = self.parent;
                self.size += 1;
            },
            (parent, None) | (None, parent) => {
                let size = 1;
                while parent != None {
                    size += circuits[parent].size; 
                    parent = circuits[parent].parent;
                }
                circuits[parent].size += size; 
                if self.parent = None {
                    self.parent = parent;
                } else {
                    other.parent = parent;
                }
            },
            (parent1, parent2) => {
                while parent1 != None {
                    parent1 = parent1.parent;
                }
                while parent2 != None {
                    parent2 = parent2.parent;
                }
                if parent1.size > parent2.size {
                    parent1.size += parent2.size;
                    parent2.parent = parent1;
                } else {
                    parent2.size += parent1.size;
                    parent1.parent = parent2;
                }
            }
        }
    }
    pub fn get_last_parent(circuit: Circuit) -> (u64, Circuit) {
        let size = 1;
        while circuit.parent == None {
            size += circuit.size;
            circuit = circuit.parent;
        }
        circuit
    }
}
