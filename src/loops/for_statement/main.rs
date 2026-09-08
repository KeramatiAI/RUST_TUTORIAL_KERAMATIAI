fn main() {
    let names = vec!["ali", "hassan", "reza"];

    for name in &names {  // این قسمت درست است (استفاده از مرجع برای جلوگیری از انتقال Ownership)
        println!("{}", name);
    }

    println!("{:?}", names);
}