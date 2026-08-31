use core::fmt;
use std::ops::Add;

#[derive(Debug)]
struct Counter;

impl Iterator for Counter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some(String::new())
    }
}

// Operator overloading using the + Operator
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Metres(u64);
#[derive(Debug)]
struct Millimetres(u64);

impl Add<Metres> for Millimetres {
    type Output = Millimetres;

    fn add(self, rhs: Metres) -> Self::Output {
        Millimetres(self.0 + rhs.0 * 1000)
    }
}

// Type specification on a trait
trait Animal {
    fn baby_name() -> String;
}

struct Dog {
    doggy: String,
}

#[allow(dead_code)]
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
    fn new(doggy: String) -> Dog {
        Dog { doggy }
    }
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.doggy.to_string())
    }
}
impl OuterlinePrint for Dog {}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

// Supertraiting your trait
// combining two traits from a super trait
trait OuterlinePrint: fmt::Display {
    fn outerline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

fn main() {
    let count = Counter;

    println!("{count:?}");

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let tall = Metres(20);

    let increase = Millimetres(30);

    let output = increase + tall;

    println!("{}", output.0);

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let doggy = Dog::new("eew".to_string());

    doggy.outerline_print();
}
