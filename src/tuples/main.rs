fn main() {
    // یک Tuple با انواع داده متفاوت
    let person: (&str, u32, bool) = ("Ali", 25, true);

    // دسترسی به اعضای Tuple با استفاده از index
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

    // Tuple تو در تو
    let coordinates: ((i32, i32), &str) = ((10, 20), "Tehran");

    println!("X: {}", coordinates.0.0);
    println!("Y: {}", coordinates.0.1);
    println!("City: {}", coordinates.1);
}
