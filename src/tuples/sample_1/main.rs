fn main() {
    let tx: (u64, f64, bool) = (1001, 0.5, false); // (id, amount, status)
    let (_id, _amount, _confirmed) = tx; // Destructuring
    println!("tx {} with amount {}", tx.0, tx.1); // Access by index
}