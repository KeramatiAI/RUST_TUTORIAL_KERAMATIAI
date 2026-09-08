fn main(){
    let balance = 1000;
    let amount = 1000;
    let status = if balance >= amount { "valid" } else { "failed" };
    println!("status:{}",status);
}