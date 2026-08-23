fn main() {
    // 1. اعداد صحیح (Integers)
    // می‌تواند با علامت (i) یا بدون علامت (u) باشد.
    let integer: i32 = -42;
    let unsigned_integer: u32 = 100;
    println!("عدد صحیح: {} و عدد صحیح بدون علامت: {}", integer, unsigned_integer);

    // 2. اعداد اعشاری (Floating-point)
    // f64 نوع پیش‌فرض است و دقت بالاتری دارد.
    let float: f64 = 3.14;
    println!("عدد اعشاری: {}", float);

    // 3. بولین (Boolean)
    let is_rust_fun: bool = true;
    println!("آیا راست جذاب است؟ {}", is_rust_fun);

    // 4. کاراکتر (Character)
    // در Rust کاراکترها با تک‌کویتی '' تعریف می‌شوند و 4 بایت هستند (Unicode)
    let letter: char = 'R';
    let emoji: char = '🚀';
    println!("حرف: {} و ایموجی: {}", letter, emoji);
}
