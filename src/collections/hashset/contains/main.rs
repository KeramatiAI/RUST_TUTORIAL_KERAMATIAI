/*
contains() Function:
Returns true if the set contains a value.
*/
use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();
    names.insert("Davoud");
    names.insert("Keramati");
    names.insert("Google");

    if names.contains(&"Google") {
        println!("found name");
    }
}