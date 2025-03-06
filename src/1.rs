use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("test.txt");
    let display = path.display();

    // Open a file in read-only mode (ignoring errors).
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);
}
