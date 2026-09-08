fn main() {
    let v = vec!["apple", "banana", "cherry"];

    // مثال 1: match ساده
    match v.get(1) {
        Some(fruit) => println!("میوه پیدا شد: {}", fruit),
        None => println!("میوه‌ای در این اندیس وجود ندارد!"),
    }

    // مثال 2: اندیس نامعتبر
    match v.get(5) {
        Some(fruit) => println!("میوه پیدا شد: {}", fruit),
        None => println!("خطا: اندیس 5 خارج از محدوده (طول: {}) است!", v.len()),
    }

    // مثال 3: استفاده از match با get_mut برای تغییر
    let mut numbers = vec![10, 20, 30];
    let index = 0;

    match numbers.get_mut(index) {
        Some(num) => {
            *num += 5;
            println!("مقدار اندیس {} به {} افزایش یافت", index, num);
        }
        None => println!("اندیس {} معتبر نیست!", index),
    }

    // مثال 4: match با چندین شرط
    let scores = vec![85, 92, 78, 95, 88];
    let student_id = 3;

    match scores.get(student_id) {
        Some(score) if *score >= 90 => println!("دانش‌آموز {} نمره عالی: {}", student_id, score),
        Some(score) if *score >= 80 => println!("دانش‌آموز {} نمره خوب: {}", student_id, score),
        Some(score) => println!("دانش‌آموز {} نمره: {}", student_id, score),
        None => println!("شناسه دانش‌آموز {} وجود ندارد!", student_id),
    }

    // مثال 5: تابع کمکی با match
    fn get_element_safely(vec: &Vec<i32>, index: usize) -> i32 {
        match vec.get(index) {
            Some(&value) => value,
            None => {
                println!("⚠️ هشدار: اندیس {} نامعتبر، مقدار پیش‌فرض 0 برگردانده شد", index);
                0
            }
        }
    }

    let data = vec![100, 200, 300];
    println!("نتیجه: {}", get_element_safely(&data, 1)); // 200
    println!("نتیجه: {}", get_element_safely(&data, 5)); // 0 با هشدار
}