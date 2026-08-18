use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap_or_default();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("here's the vector {v:?}");
    });

    handle.join().unwrap_or_default();
}
