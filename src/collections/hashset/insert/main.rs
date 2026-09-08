/*
insert() Function:
Adds a value to the set.
A HashSet does not add duplicate values
to the collection.
*/
use std::collections::HashSet;
fn main() {
    let mut names = HashSet::new();

    names.insert("Davoud");
    names.insert("Keramati");
    names.insert("IBM");
    names.insert("Keramati");//duplicates not added

    println!("{:?}",names);
}