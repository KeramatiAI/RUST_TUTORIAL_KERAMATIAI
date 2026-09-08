fn main(){
    let x = 5;
    let y = x; // ✅ No problem! The numbers are Copy
    println!("{} , {}", x, y);
}
/*
Simple types (number, bool, char, and tuple are among these types) are copied onto the stack
— there's nothing on the heap to worry about.
*/