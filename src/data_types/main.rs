fn main() {
    // 1. اعداد صحیح (Integers)
    // می‌تواند با علامت (i) یا بدون علامت (u) باشد.
    // انواع عددی رایج: i32, i64, u32, u64 (i = signed, u = unsigned، عدد = تعداد بیت)
    let integer: i32 = -42; // immutable
    let unsigned_integer: u32 = 100; // immutable
    println!("عدد صحیح: {} و عدد صحیح بدون علامت: {}", integer, unsigned_integer);

    // 2. اعداد اعشاری (Floating-point)
    // f64 نوع پیش‌فرض است و دقت بالاتری دارد.
    let float: f64 = 3.14; // immutable
    println!("عدد اعشاری: {}", float);

    // 3. بولین (Boolean)
    let is_rust_fun: bool = true; // immutable
    println!("آیا راست جذاب است؟ {}", is_rust_fun);

    // 4. کاراکتر (Character)
    // در Rust کاراکترها با تک‌کویتی '' تعریف می‌شوند و 4 بایت هستند (Unicode)
    let letter: char = 'R'; // immutable
    let emoji: char = '🚀'; // immutable
    println!("حرف: {} و ایموجی: {}", letter, emoji);
}
