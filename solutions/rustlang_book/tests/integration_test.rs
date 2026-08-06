use rustlang_book::{self, factorial::factorial, guessing_game::Guess};

#[test]
fn generic_types() {
    let list_of_numbers = vec![231, 23123, 123123, 12311, 112, 12312];
    assert_eq!(
        rustlang_book::generic_types::largest_number(list_of_numbers),
        123123
    );
}

#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}

#[test]
fn factorial_of_n() {
    match factorial(5) {
        Some(v) => assert_eq!(v, 120),
        _ => panic!("The value is not presented in the function."),
    }
}
