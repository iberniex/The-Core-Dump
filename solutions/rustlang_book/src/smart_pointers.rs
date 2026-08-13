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

#[derive(Debug, Clone)]
struct CustomSmartPointer {
    string: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.string)
    }
}

/// calling s.drop() from std::mem::drop
/// would lead to a double free error cause rust calls s.drop() anyway.
///
///```
///
/// #[derive(Debug, Clone)]
/// struct CustomSmartPointer {
///     string: String,
/// }
///
/// impl Drop for CustomSmartPointer {
///     fn drop(&mut self) {
///         println!("Dropping CustomSmartPointer with data {}", self.string)
///     }
/// }
///
/// let v = CustomSmartPointer {
/// string: String::from("Example test"),
/// };
/// drop(v);
/// ```
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

    drop(s);
    println!("The smart pointer has been dropped before the end of main.")
}
