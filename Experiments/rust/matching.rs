#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alaska,
    UmpaLumpa,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {state:?}");
            25
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

fn main() {
    println!(
        "value of a quarter: {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{six:?} - {none:?}");
    println!("{five:?}");

    let dice_roll = 5;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    };
}
