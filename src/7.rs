
fn main() {
    let x = 5;
    let y = 10;
    println!("The sum of {} and {} is {}", x, y, add(x, y));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}