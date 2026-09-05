fn add_1(num: i32) -> i32 {
    num + 1
}

// placinga function pointer for reuse
// allow you to use functions as arguments to other functions.
fn double(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// return closures using the impl keyword
pub fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
pub fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
fn main() {
    println!("{}", double(add_1, 12));

    let list_of_numbers = vec![1, 2, 3, 4];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    println!("{list_of_strings:?}");

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{list_of_status:?}");

    let handlers = vec![returns_closure(), returns_initialized_closure(56)];

    let woa = returns_closure()(5);

    println!("woa = {woa}");

    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}
