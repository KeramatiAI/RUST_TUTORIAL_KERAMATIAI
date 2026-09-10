// fn largest_i32(list: &[i32]) -> i32 {
//     let mut max = list[0];
//     for &item in list.iter() {
//         if item > max { max = item; }
//     }
//     max
// }

// T یعنی "هر نوعی" — اما با یک شرط: باید قابلیت مقایسه با > را داشته باشد
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list.iter() {
        if item > max { max = item; }
    }
    max
}

fn main() {
    let numbers = vec![34, 50, 25, 100];
    println!("بزرگترین عدد: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("بزرگترین کاراکتر: {}", largest(&chars));
}

