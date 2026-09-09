fn main() {
    let a = [10,20,30];
    a[10]; //invokes a panic since index 10 cannot be reached
}

/*
warning: this expression will panic at run-time
--> main.rs:4:4
  |
4 | a[10];
  | ^^^^^ index out of bounds: the len is 3 but the index is 10

$main
thread 'main' panicked at 'index out of bounds: the len
is 3 but the index is 10', main.rs:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
*/