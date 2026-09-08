/*
Insert Function:
Inserts a key/value pair into the HashMap.
*/
use std::collections::HashMap;
fn main(){
    let mut stateCodes = HashMap::new();
    stateCodes.insert("name","Davoud");
    stateCodes.insert("family","Keramati");
    stateCodes.insert("age","30");
    stateCodes.insert("salary","10000$");
    stateCodes.insert("weight","80kg");
    println!("{:?}",stateCodes);
}