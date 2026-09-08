fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // مثال 1: استفاده ساده با unwrap_or
    let value = v.get(2).unwrap_or(&0);
    println!("مقدار در اندیس 2: {}", value); // خروجی: 30

    // مثال 2: اندیس نامعتبر با مقدار پیش‌فرض
    let value = v.get(10).unwrap_or(&0);
    println!("مقدار در اندیس 10: {}", value); // خروجی: 0

    // مثال 3: استفاده در محاسبات
    let index = 3;
    if let Some(val) = v.get(index) {
        println!("مقدار پیدا شد: {} و دو برابر آن: {}", val, val * 2);
    } else {
        println!("اندیس {} معتبر نیست!", index);
    }

    // مثال 4: زنجیره‌ای با عملیات دیگر
    let result = v.get(1)
        .map(|x| x * 10)
        .unwrap_or(100);
    println!("نتیجه: {}", result); // خروجی: 200
}