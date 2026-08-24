fn main() {
    let number = 15;

    // ۱. استفاده ساده از if/else
    if number < 10 {
        println!("عدد کوچک است");
    } else if number == 15 {
        println!("عدد دقیقاً ۱۵ است");
    } else {
        println!("عدد بزرگ است");
    }

    // ۲. استفاده از if به عنوان یک Expression (بسیار کاربردی!)
    // در Rust می‌توانید نتیجه یک if را مستقیماً در یک متغیر بریزید.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("مقدار عدد بر اساس شرط: {}", number);
}
