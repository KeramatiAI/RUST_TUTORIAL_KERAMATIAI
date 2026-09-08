fn main() {
    // Vector: a variable-length array on the heap
    let mut chain: Vec<String> = Vec::new();
    chain.push(String::from("Genesis Block"));
    chain.push(String::from("Block #1"));
    println!("First block: {}", chain[0]);

    // vec macro
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);

    // String: can grow and change
    let mut s = String::from("Hello ");
    s.push_str("World");

    // Concatenation
    let _s3 = String::from("Block") + &"Chain";
}