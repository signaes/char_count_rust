use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    for word in args.iter() {
        println!("'{}' count = {:?}", word, count(&word));
    }
}

fn count(s: &String) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();

    for c in s.chars() {
        *map.entry(c.to_string()).or_insert(0) += 1;
    }

    map
}
