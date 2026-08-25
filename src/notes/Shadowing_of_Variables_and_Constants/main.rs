fn main() {
    let salary = 100.00;
    // let salary = 25.00;
    // reads first salary
    println!("The value of salary is :{}", salary);

    let uname = "Mohtashim";
    let uname = uname.len();
    println!("name changed to integer : {}",uname);

    const NAME:&str = "Mohtashim";
    const NAME:usize = NAME.len();
    //Error : `NAME` already defined
    println!("name changed to integer : {}",NAME);
}