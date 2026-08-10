fn main() {
    let factorial = factorial(3);
    match factorial {
        Some(val) => println!("The value is {}", val),
        None => println!("The value is not found oops!"),
    }

    let factorial_cleany = factorial_clean(5);

    println!("The clean factorial suggested by Clippy is {factorial_cleany}")
}

fn factorial(n: u32) -> Option<u32> {
    (1..=n).try_fold(1u32, |acc, x| acc.checked_mul(x))
}

fn factorial_clean(n: u32) -> u32 {
    (1..=n).product()
}
