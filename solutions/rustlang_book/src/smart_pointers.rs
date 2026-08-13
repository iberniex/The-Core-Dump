use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementation for the Deference trait on MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Drop Trait
// Implementation of the Drop Trait on a structure CustomSmartPointer

#[derive(Debug)]
struct CustomSmartPointer {
    string: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.string)
    }
}

pub fn testing_box_smart_pointer() {
    let s = MyBox::new(5);

    let r = Box::new(6);

    println!("{:?}", s);
    println!("{:?}", r);

    println!("{}", *s);
    println!("{}", *r);

    let s = CustomSmartPointer {
        string: String::from("Ready"),
    };

    println!("Smart Pointer {s:?} Created")
}
