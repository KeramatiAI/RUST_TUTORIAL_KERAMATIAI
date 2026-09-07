/*
Syntax

//public module
pub mod a_public_module {
   pub fn a_public_function() {
      //public function
   }
   fn a_private_function() {
      //private function
   }
}
//private module
mod a_private_module {
   fn a_private_function() {
   }
}
*/
pub mod movies {
    pub fn play(name:String) {
        println!("Best Programming Language for Blockchain is: {}",name);
    }
}
fn main(){
    movies::play("RUST".to_string());
}