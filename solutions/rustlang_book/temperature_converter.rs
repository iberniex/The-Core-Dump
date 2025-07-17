use std::io;

fn main() {
    println!("Welcome to temperature converter.\n");
    println!("Please choose the version you want to convert\n");

    println!("1. Celsius\n2. Fahrenheit\n");

    loop {
        println!("Input your choice");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The input inserted: {choice}");

        let mut number = String::new();
        if choice == 1 {
            println!("Insert your number");

            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read line");

            println!("Converting {number} to Fahrenheit");

            let number: f32 = match number.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("Result is {}", celsius2_fah(number));
            break;
        } else {
            println!("Insert your number");

            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read line");

            println!("Converting {number} to celsius");

            let number: f32 = match number.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("Result is {}", fah2_celsius(number));
            break;
        }
    }
}

fn fah2_celsius(num: f32) -> f32 {
    (num - 32.0) / 1.8
}

fn celsius2_fah(num: f32) -> f32 {
    num * 1.8 + 32.0
}
