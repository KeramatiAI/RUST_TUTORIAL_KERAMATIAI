fn main() {
    // 1. نوع &str (String Slice)
    // این‌ها معمولاً رشته‌های ثابت هستند که در زمان کامپایل در حافظه برنامه قرار می‌گیرند.
    let fixed_string: &str = "سلام، این یک رشته ثابت است";
    println!("{}", fixed_string);

    // 2. نوع String (رشته داینامیک)
    // این نوع در Heap ذخیره می‌شود و می‌توانید آن را تغییر دهید (اضافه یا کم کنید).
    let mut dynamic_string = String::from("Rust");

    // اضافه کردن یک کاراکتر یا رشته به انتهای آن
    dynamic_string.push_str(" خیلی عالی است!");
    println!("{}", dynamic_string);

    // 3. تبدیل &str به String و برعکس
    let s1 = "تبدیل ثابت به داینامیک".to_string();
    let s2: &str = &dynamic_string; // گرفتن Slice از String (بسیار رایج)

    println!("s1: {}, s2: {}", s1, s2);
}
