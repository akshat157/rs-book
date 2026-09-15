use std::thread;

fn main() {
    let v1 = vec![1, 2, 3];
    let t1 = thread::spawn(move || {
        println!("Here's the vector from thread: {v1:?}");
    });

    t1.join().unwrap();
}
