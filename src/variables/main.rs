fn main() {
    // Define an immutable variable (default)
    let x = 5;
    println!("The value of the variable x is: {}", x);

    // If we try to change x (for example x = 6;) we will encounter an error.

    // To define a mutable variable (Mutable), we use the mut keyword
    let mut y = 10;
    println!("Initial value of y: {}", y);

    y = 20; // Now we are allowed to change
    println!("New value of y: {}", y);
}
