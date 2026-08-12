use rustlang_book::{
    generic_types::largest_number, lifetimes::largest, smart_pointers::testing_box_smart_pointer,
};
fn main() {
    let list_of_nums = vec![23, 342, 123, 12312, 31223];
    let largest_number = largest_number(list_of_nums);
    let largest_string = largest("ready", "reaaaddyyyyy");
    println!("This is the largest string - {}", largest_string);
    println!("The large number in {}", largest_number);

    // guessing_game::guessing_game();

    // worker();

    // closure_runner();

    testing_box_smart_pointer();
}
