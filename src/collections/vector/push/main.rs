/*
Push Function:
Appends an element to the end of a collection.
*/
fn main() {
    let mut info_vector = Vec::new();
    info_vector.push("Davoud");
    info_vector.push("Keramati");
    info_vector.push("1234567890");
    info_vector.push("true");
    info_vector.push("57.640kg");

    println!("{:?}",info_vector);
}