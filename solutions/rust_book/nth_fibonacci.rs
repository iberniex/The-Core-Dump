use std::io;

fn main() {
    let mut number = String::new();

    println!("Insert number to find out\nThe nth fibonacci number");

    io::stdin().read_line(&mut number).expect("Failed readline");

    let number = number.trim().parse().unwrap_or(0);

    println!(
        "The nth fibonacci number is {}",
        iterative_fibonacci(number)
    );
}

fn _fibonacci(num: i64) -> i64 {
    if num < 2 {
        num
    } else {
        _fibonacci(num - 1) + _fibonacci(num - 2)
    }
}

// iterative fibonacci
fn iterative_fibonacci(num: i64) -> i64 {
    if num < 2 {
        return num;
    }

    let (mut a, mut b) = (0, 1);
    for _ in 2..=num {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
