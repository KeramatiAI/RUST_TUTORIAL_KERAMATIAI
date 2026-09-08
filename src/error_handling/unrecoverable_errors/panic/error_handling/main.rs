fn main() {
    let index = 10;
    let v = vec![1, 2, 3];

    if index >= v.len() {
        panic!("خطا: اندیس {} خارج از محدوده وکتور با طول {} است", index, v.len());
    }

    let value = v[index];
    println!("{}", value);
}