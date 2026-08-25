fn main(){
    // If Statement
    let num:i32 = 5;
    if num > 0 {
        println!("number is positive");
    }
    // If Else Statement
    let num2 = 12;
    if num2 % 2==0 {
        println!("Even");
    } else {
        println!("Odd");
    }
    // Nested If Statement
    let num3 = 2 ;
    if num3 > 0 {
        println!("{} is positive",num3);
    } else if num3 < 0 {
        println!("{} is negative",num3);
    } else {
        println!("{} is neither positive nor negative",num) ;
    }
    // Match Statement
    let state_code = "MH";
    let state = match state_code {
        "MH" => {println!("Found match for MH"); "Maharashtra"},
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "Goa",
        _ => "Unknown"
    };
    println!("State name is {}",state);
}