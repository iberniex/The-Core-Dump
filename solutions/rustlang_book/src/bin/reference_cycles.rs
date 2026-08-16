use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, value) => Some(value),
            List::Nil => None,
        }
    }
}
fn main() {
    let a = Rc::new(List::Cons(10, RefCell::new(Rc::new(List::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List::Cons(5, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));

    println!("b next item = {:?}", b.tail());

    // This is the reference cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // This overflows the stack due to the reference in a.tail to be b.tail which leads
    // to an endless loop
    // println!("a next item = {:?}", a.tail());
}
