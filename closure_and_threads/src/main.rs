use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure. List: {list:?}");

    thread::spawn(move || println!("From thread. {list:?}"))
        .join()
        // Check https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures
        .unwrap_or_else(|e| println!("Error occured while joining the thread: {e:?}"));
}
