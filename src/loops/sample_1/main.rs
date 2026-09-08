fn main(){
    println!("------ loop ------");
    // loop: infinite loop with return value
    let mut hash_attempts = 0;
    let _found = loop {
        hash_attempts += 1;
        if hash_attempts == 100 {
            break hash_attempts; // loop can return value!
        }
    };
    println!("found in:{}",_found);

    println!("------ while ------");
    // while
    let mut n = 3;
    while n != 0 {
        println!("{}", n);
        n -= 1;
    }
    println!("------ for ------");
    // for: most common and safest method
    for i in 0..5 { // 0,1,2,3,4
        println!("Block number {}", i);
    }
    println!("------ for ------");
    for block in [10, 20, 30].iter() {
        println!("{}", block);
    }
}