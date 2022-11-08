use std::collections::HashMap;
use std::{vec::Vec};

fn main() {
    let mut my_hash: HashMap<i32, &str> = HashMap::new();

    for i in 1..9 {
        my_hash.insert(i, "sdas");
    }
    let mut count_vec: Vec<_> = my_hash.iter().collect();
    count_vec.sort_by(|x,y| x.0.cmp(&y.0));
    for (k, v)in count_vec {
        println!("{}: {}", k, v)
    }

}