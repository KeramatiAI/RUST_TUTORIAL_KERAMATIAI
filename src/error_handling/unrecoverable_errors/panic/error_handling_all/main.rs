fn main() {
    let mut inventory = vec!["item1", "item2", "item3", "item4"];

    // تابع برای دریافت ایمن با get
    fn get_item(inv: &Vec<&str>, idx: usize) -> String {
        inv.get(idx)
            .map(|&item| format!("✅ آیتم پیدا شد: {}", item))
            .unwrap_or_else(|| format!("❌ آیتم با اندیس {} وجود ندارد!", idx))
    }

    // تابع برای تغییر ایمن با get_mut
    fn update_item<'a>(inv: &mut Vec<&'a str>, idx: usize, new_value: &'a str) -> bool {
        match inv.get_mut(idx) {
            Some(item) => {
                *item = new_value;
                println!("🔧 آیتم اندیس {} به '{}' تغییر یافت", idx, new_value);
                true
            }
            None => {
                println!("❌ خطا: اندیس {} خارج از محدوده!", idx);
                false
            }
        }
    }

    // استفاده از توابع
    println!("{}", get_item(&inventory, 2));
    println!("{}", get_item(&inventory, 10));

    update_item(&mut inventory, 1, "updated_item");
    update_item(&mut inventory, 20, "invalid");

    println!("وضعیت نهایی: {:?}", inventory);
}