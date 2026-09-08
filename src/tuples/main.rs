fn main() {
    // A tuple with different data types
    let person: (&str, u32, bool) = ("Ali", 25, true);

    // Accessing tuple members using index
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

    // Nested tuple
    let coordinates: ((i32, i32), &str) = ((10, 20), "Tehran");

    println!("X: {}", coordinates.0.0);
    println!("Y: {}", coordinates.0.1);
    println!("City: {}", coordinates.1);
}
