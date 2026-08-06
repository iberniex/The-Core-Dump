use std::fmt::Display;

pub fn largest<'a>(string_1: &'a str, string_2: &'a str) -> &'a str {
    if string_1.len() > string_2.len() {
        return string_1;
    }

    string_2
}

pub fn largest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement {}", ann);

    if x.len() > y.len() { x } else { y }
}
