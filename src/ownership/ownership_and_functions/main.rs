fn main() {
    let s = String::from("block");
    takes_ownership(s); // Ownership passed to function

    // println!("{}", s); ❌ s is no longer valid!

    let num = 5;
    makes_copy(num); // ✅ because i32 is copied
    println!("{}", num); // ✅ still valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // here some_string is out of scope and memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}