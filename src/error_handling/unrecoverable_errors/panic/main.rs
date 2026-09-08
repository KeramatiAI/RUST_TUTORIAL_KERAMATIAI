fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let index = 10;

    // Check if the index is in the allowed range
    if index >= v.len() {
        panic!("Out of band error! Index {} is invalid for vector of length {}", index, v.len());
    }

    // If we reach this line, the index is valid
    let element = &v[index];
    println!("Element at index {}: {}", index, element);
}
