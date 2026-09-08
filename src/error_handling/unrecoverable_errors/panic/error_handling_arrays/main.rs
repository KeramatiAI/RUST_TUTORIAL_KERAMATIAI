fn main() {
    let arr: [i32; 3] = [10, 20, 30];

    // این خط باعث panic می‌شود چون اندیس 3 خارج از محدوده (0..2) است
    let element = arr[3];
    println!("{}", element);
}