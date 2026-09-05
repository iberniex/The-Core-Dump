// Declarative macros
// Mostly used macros
// follow the macro_rules syntax
#[macro_export]
macro_rules! vecy {
    ( $( $x:expr ),*) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
            )*

            drop(temp_vec)
    };
}

// derivative macros (derive macros)
pub trait HelloMacro {
    fn hello_macro();
}

#[derive(Debug)]
#[allow(dead_code)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();

    vecy!(1, 2, 3);
}
