fn main() {
    let mut mutable_value:i32 = 25_000; // mutable
    println!("mutable_value is {} ",mutable_value);
    mutable_value = 35_000;
    println!("mutable_value changed is {}",mutable_value);

    let immutable_value = 100_000;
    println!("immutable_value is {}",immutable_value);
    immutable_value = 500_000; // error, reassignment of immutable variable
}