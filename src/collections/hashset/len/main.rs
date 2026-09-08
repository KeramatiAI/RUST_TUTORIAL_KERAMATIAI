/*
len() Function:
Returns the number of elements in the set.
*/
use std::collections::HashSet;
fn main() {
    let mut names = HashSet::new();
    names.insert("Davoud");
    names.insert("Keramati");
    names.insert("DeepMind");
    println!("size of the set is {}",names.len());
}