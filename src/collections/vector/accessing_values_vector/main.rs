/*
Accessing Values from a Vector
Individual elements in a vector
can be accessed using their corresponding index numbers.
The following example creates a vector ad prints the value of the first element.
*/

fn main() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);

    println!("{:?}",v[0]);
}