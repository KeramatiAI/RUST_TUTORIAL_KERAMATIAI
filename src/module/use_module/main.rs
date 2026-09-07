/*
Syntax:
use public_module_name::function_name;
*/
pub mod movies {
    pub fn play(name:String) {
        println!("Best Programming Language for Blockchain is: {}",name);
    }
}
use movies::play;
fn main(){
    play("RUST".to_string());
}