/*
Syntax:
let mut instance_name = Vec::new();

let vector_name = vec![val1,val2,val3]
*/

fn main() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);

    println!("size of vector is :{}",v.len());
    println!("{:?}",v);
}