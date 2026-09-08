/*
Len Function:
Returns the number of elements in the map
*/
use std::collections::HashMap;
fn main() {
    let mut stateCodes = HashMap::new();
    stateCodes.insert("KL","Kerala");
    stateCodes.insert("MH","Maharashtra");
    println!("size of map is {}",stateCodes.len());
}