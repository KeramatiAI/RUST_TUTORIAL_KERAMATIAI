/*
enum Result<T,E> {
   OK(T),
   Err(E)
}
*/

/*
use std::fs::File;
fn main() {
   let f = File::open("main.jpg");
   //this file does not exist
   println!("{:?}",f);
}
*/

use std::fs::File;
fn main() {
    let f = File::open("main.jpg");   // main.jpg doesn't exist
    match f {
        Ok(f)=> {
            println!("file found {:?}",f);
        },
        Err(e)=> {
            println!("file not found \n{:?}",e);   //handled error
        }
    }
    println!("end of main");
}