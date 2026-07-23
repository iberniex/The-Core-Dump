fn main() {
    let factorial = factorial(3);
    match factorial {
        Some(val) => println!("The value is {}", val),
        None => println!("The value is not found oops!"),
    }
}

fn factorial(n: u32) -> Option<u32> {
    (1..=n).try_fold(1u32, |acc, x| acc.checked_mul(x))
}
