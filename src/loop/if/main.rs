fn main(){
    for x in 1..10{
        if x==5 {
            continue;
        }
        println!("X is:{}",x);
    }
}