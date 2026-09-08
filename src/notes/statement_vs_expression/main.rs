fn main(){
    println!("{}",example());
}
fn example() -> u64 {
    let x = 5;
    x + 1 // ✅ Expression without semicolon = return value
    // x + 1; ❌ If you put semicolon, you will get an error
}