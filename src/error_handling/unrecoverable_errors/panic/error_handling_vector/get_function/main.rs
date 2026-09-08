fn main() {
    let v = vec![1, 2, 3];

    // روش ایمن - بدون panic
    match v.get(5) {
        Some(value) => println!("{}", value),
        None => println!("اندیس خارج از محدوده است!"),
    }

    // یا با unwrap_or برای مقدار پیش‌فرض
    let value = v.get(5).unwrap_or(&0);
    println!("{}", value);
}