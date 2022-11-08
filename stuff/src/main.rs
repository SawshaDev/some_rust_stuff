use std::vec::Vec;

fn main() {
    let mut list: Vec<i32> = Vec::new();

    for i in 1..11 {
        list.push(i)
    }

    for elem in list.iter() {
        println!("{}", elem)
    }
}   