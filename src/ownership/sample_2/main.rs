fn main(){
    let x = 5;
    let y = x; // ✅ No problem! The numbers are Copy
    println!("{} , {}", x, y);
}