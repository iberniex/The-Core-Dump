fn main() {
    let s1 = String::from("hello");

    let (s2, len) = len_calculate(s1);

    println!("This string {s2} is of len:{len}");
}

fn len_calculate(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
