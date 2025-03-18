use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    println!("{:?}", map);
}
