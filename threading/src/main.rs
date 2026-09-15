use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let msgs = vec![
            String::from("hi!"),
            String::from("from"),
            String::from("thread"),
            String::from("t1"),
        ];

        for msg in msgs {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("some"),
            String::from("message"),
            String::from("from"),
            String::from("t2"),
        ];

        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(800));
        }
    });

    for received_msg in rx {
        println!("received: \"{received_msg}\"");
    }
}
