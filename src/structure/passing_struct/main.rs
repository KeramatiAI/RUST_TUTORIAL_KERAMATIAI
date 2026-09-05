//declare a structure
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
    //initialize a structure
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
        age: 20,
        average: 10.25,
        academic_standing: String::from("Bad"),
    };
    //pass emp1 and emp2 to display()
    display(std1);
    display(std2);
}
// fetch values of specific structure fields using the
// operator and print it to the console
fn display( std: Student){
    println!("First Name is :{} Last Name is {} Student NO is {} Date of Entery is {} age is {} average is {} academic standing is {}",
             std.first_name,
             std.last_name,
             std.student_number,
             std.date_of_entry,
             std.age,
             std.average,
             std.academic_standing);
}