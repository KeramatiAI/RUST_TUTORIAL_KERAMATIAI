// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.

#[derive(Debug)]
enum GenderCategory {
    Male,Female
}

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Person<'a> {
    name:&'a String,
    gender:GenderCategory
}

fn main() {
    let p1 = Person {
        name:&String::from("Davoud"),
        gender:GenderCategory::Male
    };
    let p2 = Person {
        name:&String::from("Aysan"),
        gender:GenderCategory::Female
    };
    println!("{:#?}", p1);
    println!("{:#?}", p2);
}