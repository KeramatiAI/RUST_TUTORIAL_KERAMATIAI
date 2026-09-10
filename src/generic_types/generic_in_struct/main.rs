// یک تراکنش کلی — نوع مبلغ (T) می‌تواند u64 یا f64 یا هر چیزی باشد
struct Transaction<T> {
    id: u64,
    amount: T,
    sender: String,
    receiver: String,
}

// یک بلاک که داده‌ی تراکنش‌هایش (T) جنریک است
struct Block<T> {
    index: u64,
    data: Vec<T>,          // چند تراکنش در هر بلاک
    previous_hash: String,
}

fn main() {
    let tx1 = Transaction { id: 1, amount: 50u64, sender: "A".into(), receiver: "B".into() };
    let tx2 = Transaction { id: 2, amount: 5.5f64, sender: "B".into(), receiver: "C".into() };

    let block = Block { index: 0, data: vec![tx1, tx2], previous_hash: "0000".into() };
    println!("بلاک #{} با {} تراکنش", block.index, block.data.len());
}
