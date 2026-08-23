fn main() {
    // آرایه‌ای از اعداد صحیح
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    // دسترسی به اعضای آرایه
    println!("First number: {}", numbers[0]);
    println!("Third number: {}", numbers[2]);

    // چاپ کل آرایه
    println!("Numbers: {:?}", numbers);

    // تعداد اعضای آرایه
    println!("Length: {}", numbers.len());

    // آرایه با مقدار تکرارشونده
    let zeros = [0; 4];

    println!("Zeros: {:?}", zeros);
}
