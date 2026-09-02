use std::thread;

fn main() {
    /// Never use mutable static values they live for entirety
    /// of the program call
    static mut STATIC: i32 = 55;

    thread::spawn(move || unsafe {
        // WARNING: don't do this 
        STATIC += 1;
    })


    let reddy = STATIC;
    // NOTE: You can't reference static values cause it's denied by default

    println!("{reddy}");
}
