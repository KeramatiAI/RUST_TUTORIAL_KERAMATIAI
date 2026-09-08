/*
remove() Function:
Removes a value from the set.
*/
use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();
    names.insert("Dacoud");
    names.insert("Keramati");
    names.insert("OpenAI");
    println!("length of the Hashset: {}",names.len());
    names.remove(&"Keramati");
    println!("length of the Hashset after remove() : {}",names.len());
}