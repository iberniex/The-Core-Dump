use std::io;

pub fn improved_temperature_converter() {
    println!("Welcome to temperature converter.\n");

    loop {
        println!("Please choose the version you want to convert\n");

        println!("1. Celsius");
        println!("2. Fahrenheit\n");
        println!("3. Exit");

        let choice = get_user_input("Please insert your input: ");

        match choice.trim() {
            "1" => {
                let celsius = get_float_input("Please insert your celsius temp: ");
                let fahernheit = celsius * 1.8 + 32.0;
                println!("celsius {celsius:.2} -> {fahernheit:.2}");
            }
            "2" => {
                let fahernheit = get_float_input("Please insert your fahrenheit temp: ");
                let celsius = (fahernheit - 32.0) / 1.8;
                println!("celsius {fahernheit:.2} -> {celsius:.2}");
            }
            "3" => {
                println!("Thank you for using the app!");
                break;
            }
            _ => {
                println!("Invalid choice!");
            }
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{prompt}");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    choice
}

fn get_float_input(prompt: &str) -> f64 {
    loop {
        let input = get_user_input(prompt);

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        };
    }
}
