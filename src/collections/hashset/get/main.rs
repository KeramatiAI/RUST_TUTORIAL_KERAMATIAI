/*
get() Function:
Returns a reference to the value in the set, if any, which is equal to the given value.
*/
use std::collections::HashSet;
fn main() {
    let mut names = HashSet::new();
    names.insert("Mohtashim");
    names.insert("Kannan");
    names.insert("TutorialsPoint");
    names.insert("Mohtashim");

    match names.get(&"Mohtashim"){
        Some(value)=>{
            println!("found {}",value);
        }
        None =>{
            println!("not found");
        }
    }
    println!("{:?}",names);
}