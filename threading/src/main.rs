use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("message writen by thread t1");

        println!("t1 is sending: \"{msg}\"");
        tx.send(msg).unwrap();

        // The following statement gives compilation error as msg
        // has been moved out of this closure in the tx.send() call.
        // println!("t1 sent the following: {msg}");
    });

    let received_msg = rx.recv().unwrap();
    println!("thread main received: \"{received_msg}\"");
}
