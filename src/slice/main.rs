fn main() {
    let str = "Davoud Keramati is a programer".to_string();
    println!("length of string is {}",str.len());
    let slice = &str[4..9];

    // fetches characters at 4,5,6,7, and 8 indexes
    println!("{}",slice);
}