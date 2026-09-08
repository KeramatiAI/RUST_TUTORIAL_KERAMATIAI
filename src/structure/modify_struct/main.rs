//Declaring a structure
struct Employee {
    name:String,
    company:String,
    age:u32
}

fn main() {
    //Initializing a structure
    let mut emp1 = Employee {
        company:String::from("OpenAI"),
        name:String::from("Davoud"),
        age:50
    };
    //Modifying a struct instance
    emp1.age = 40;
    println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
}