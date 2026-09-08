fn main() {
    let s1 = String::from("blockchain");
    let s2 = s1; // ⚠️ Ownership of s1 has been moved to s2!

    // println!("{}", s1); ❌ Error: s1 is no longer valid
    println!("{}", s2); // ✅
}
/*
String data is on the heap.
If Rust allows two pointers to the same memory → double free (classic C++ bug). Rust solves this with Move.
*/