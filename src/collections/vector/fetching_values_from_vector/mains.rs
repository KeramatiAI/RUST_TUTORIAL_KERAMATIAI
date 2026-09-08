/*
Values in a vector can also be fetched using reference to the collection.
*/
fn main() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(500);

    for i in &v {
        println!("{}",i);
    }
    println!("{:?}",v);
}