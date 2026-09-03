struct Student {
    first_name: String,
    last_name: String,
    national_code: String,
    student_no: String,
    age: u32,
    average: f32,
    address: String,
}

fn main() {
    let student = Student {
        first_name: String::from("Davoud"),
        last_name: String::from("Keramati"),
        national_code: String::from("0010548752"),
        student_no: String::from("92041201"),
        age: 40,
        average: 19.25,
        address: String::from("Tehran, Azadi stadium St, Manoochehri, P12"),
    };

    // چاپ تمام اطلاعات
    println!("First Name: {}", student.first_name);
    println!("Last Name: {}", student.last_name);
    println!("National Code: {}", student.national_code);
    println!("Student No: {}", student.student_no);
    println!("Age: {}", student.age);
    println!("Average: {}", student.average);
    println!("Address: {}", student.address);
}