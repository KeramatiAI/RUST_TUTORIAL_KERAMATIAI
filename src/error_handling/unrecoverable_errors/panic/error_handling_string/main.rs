fn main() {
    let s = String::from("سلام");
    let bytes = s.as_bytes();

    // اندیس 10 خارج از محدوده بایت‌ها است
    let byte = bytes[10];
    println!("{}", byte);
}