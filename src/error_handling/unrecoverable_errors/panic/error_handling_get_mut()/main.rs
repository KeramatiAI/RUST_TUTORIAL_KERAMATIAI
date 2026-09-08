fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // مثال 1: تغییر مقدار در اندیس معتبر
    if let Some(val) = v.get_mut(2) {
        *val = 100; // مقدار اندیس 2 را به 100 تغییر می‌دهیم
        println!("مقدار اندیس 2 به {} تغییر یافت", val);
    }

    // مثال 2: تغییر با اندیس نامعتبر
    if let Some(val) = v.get_mut(10) {
        *val = 999;
        println!("تغییر انجام شد!");
    } else {
        println!("خطا: اندیس 10 خارج از محدوده است!");
    }

    // مثال 3: تغییر چند مقدار به صورت زنجیره‌ای
    for i in 0..v.len() {
        if let Some(val) = v.get_mut(i) {
            *val *= 2; // همه مقادیر را دو برابر می‌کنیم
        }
    }

    println!("وکتور نهایی: {:?}", v); // خروجی: [2, 4, 200, 8, 10]

    // مثال 4: تغییر شرطی با get_mut
    let mut data = vec![5, 15, 25, 35];
    let target_index = 1;

    match data.get_mut(target_index) {
        Some(value) => {
            if *value > 10 {
                *value = 0; // اگر مقدار بیشتر از 10 بود، صفر کن
                println!("مقدار اندیس {} به صفر تغییر کرد", target_index);
            }
        }
        None => println!("اندیس {} خارج از محدوده است", target_index),
    }

    println!("دیتا نهایی: {:?}", data); // خروجی: [5, 0, 25, 35]
}