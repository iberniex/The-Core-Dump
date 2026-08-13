use std::{ops::Deref, rc::Rc, sync::Arc, thread};

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

enum List {
    Cons(i32, Rc<List>),
    Nil,
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

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = List::Cons(3, Rc::clone(&a));

    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = List::Cons(2, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("counf after c goes out of scope = {}", Rc::strong_count(&a));

    let s = CustomSmartPointer {
        string: String::from("Ready"),
    };

    drop(s);
    println!("The smart pointer has been dropped before the end of main.")
}

// Shared ownership between threads that work concurrently I guess?
// ARC = Automatic Reference Counters
pub fn learning_on_arc_smart_pointer() {
    let numbers: Vec<_> = (0..100u32).collect();

    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();
    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|n| **n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap()
    }
}
