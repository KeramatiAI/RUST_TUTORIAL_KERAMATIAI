/*
Remove Function:
Removes and returns the element
at position index within the vector,
shifting all elements after it to the left.
*/
fn main() {
    let mut v = vec![10,20,30];
    v.remove(1);
    println!("{:?}",v);
}