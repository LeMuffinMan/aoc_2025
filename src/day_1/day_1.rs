use crate::day_1::dial::Dial;

pub fn solve_day_1(input: Vec<String>) {
    let mut dial = Dial {
        value: 50,
        password: 0,
    };

    for rot in &input {
        let (dir_char, clicks) = rot.split_at(1);
        let dir = dir_char.parse().unwrap();
        let num: i32 = clicks.parse().unwrap();
        dial.rotate(dir, num);
        // println!("{:?} | {:?}", dial, rot);
    }
    println!("password = {:?}", dial.password);
}
