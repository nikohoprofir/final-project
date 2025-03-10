
// Rust code to generate a random number between 1 and 6
fn main() {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1..=6);
    println!("You rolled a {}", roll);
}