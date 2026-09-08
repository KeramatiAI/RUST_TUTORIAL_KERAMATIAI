/*
Contains Function:
Returns true if the slice contains
an element with the given value −
*/
fn main() {
    let v = vec![10,20,30];
    if v.contains(&10) {
        println!("found 10");
    }
    println!("{:?}",v);
}