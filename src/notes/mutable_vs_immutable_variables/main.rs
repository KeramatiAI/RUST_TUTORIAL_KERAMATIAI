fn main() {
    let x = 5; // Variable x is immutable
    // x = 6; // If you uncomment this line, the program will give an error!

    let mut y_mutable = 10; // With the mut keyword, the variable becomes mutable
    y_mutable = 15;

    println!("The value of y is equal to: {}", y_mutable);
}
