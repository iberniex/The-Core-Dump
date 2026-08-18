use std::{sync::mpsc::channel, thread, time::Duration, vec};

fn main() {
    let (tx, rx) = channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = vec![
            String::from("hi"),
            String::from("from"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in val {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let val = vec![
            String::from("here!"),
            String::from("more"),
            String::from("messages"),
            String::from("here"),
        ];

        for val in val {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("I got {received}");
    }
}
