fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1, 10);
    println!("The random number is: {}", random_number);
}
