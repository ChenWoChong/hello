use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    explain("a", map);
}

fn explain<K, V>(s: &str, map: HashMap<K, V>) {
    println!("{} {} {}", s, map.len(), map.capacity());
}
