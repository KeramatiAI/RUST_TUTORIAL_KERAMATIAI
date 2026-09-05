// declare a structure
struct Student {
    first_name: String,
    last_name: String,
    student_number: String,
    date_of_entry: String,
    age: u8,
    average: f32,
    academic_standing: String,
}

fn main() {
    // initialize structures
    let std1 = Student {
        first_name: String::from("Davoud"),
        last_name: String::from("Keramati"),
        student_number: String::from("9812300"),
        date_of_entry: String::from("2022-09-20"),
        age: 20,
        average: 18.5,
        academic_standing: String::from("Excellent"),
    };

    let std2 = Student {
        first_name: String::from("Ali"),
        last_name: String::from("Alavi"),
        student_number: String::from("9812345"),
        date_of_entry: String::from("2023-09-20"),
        age: 22, // تغییر سن برای تست درست بودن تابع
        average: 10.25,
        academic_standing: String::from("Bad"),
    };

    // استفاده از & به جای انتقال مالکیت (Borrowing)
    display(&std1);
    display(&std2);

    // مقایسه سن با استفاده از ارجاع
    // خروجی این تابع یک ارجاع به بزرگترین فرد است
    let elder = who_is_elder(&std1, &std2);

    println!("\nElder is:");
    display(elder);
}

// استفاده از &Student به جای Student برای جلوگیری از Move شدن
fn display(std: &Student) {
    println!(
        "First Name: {} | Last Name: {} | Student NO: {} | Entry Date: {} | Age: {} | Average: {} | Standing: {}",
        std.first_name,
        std.last_name,
        std.student_number,
        std.date_of_entry,
        std.age,
        std.average,
        std.academic_standing
    );
}

// این تابع حالا ارجاع (&Student) می‌گیرد و ارجاع برمی‌گرداند
fn who_is_elder<'a>(std1: &'a Student, std2: &'a Student) -> &'a Student {
    if std1.age > std2.age {
        std1
    } else {
        std2
    }
}
