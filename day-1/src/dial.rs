
#[derive(Debug)]
pub struct Dial {
    pub value: i32,
    pub password: u32,
}

impl Dial {
    pub fn rotate(&mut self, dir: char, mut clicks: i32) {
        loop {
            match dir {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => unreachable!(),
            };
            clicks -= 1;
            if clicks == 0 { break };
        }
    }
    pub fn turn_left(&mut self) {
        if self.value == 0 {
            self.value = 99;
        } else {
            self.value -= 1;
            if self.value == 0 {
                self.password += 1;
            }
        }
    }
    pub fn turn_right(&mut self) {
        if self.value == 99 {
            self.value = 0;
            self.password += 1;
        } else {
            self.value += 1;
        }
    }
}
