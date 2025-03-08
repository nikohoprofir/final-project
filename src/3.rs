use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn main() {
    let mut file = File::create("numbers.txt").unwrap();
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let num: u32 = rng.gen_range(1, 10);
        file.write_all(&num.to_string().as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}
