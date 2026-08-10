use std::{thread, time::Duration};

#[derive(PartialEq, Clone, Copy, Debug)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_shirt: u32 = 0;
        let mut blue_shirt: u32 = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_shirt += 1,
                ShirtColor::Blue => blue_shirt += 1,
            }
        }

        if red_shirt > blue_shirt {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", cached_result.value(intensity));
        println!("Next, do {} pushups", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!("Today, run for {} minutes.", cached_result.value(intensity));
        }
    }
}
pub fn closure_runner() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2: Option<ShirtColor> = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let mut list = vec![1, 2, 3, 4, 5];

    println!("This is list{:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure {list:?}");

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut rect_list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations_counter = 0;

    rect_list.sort_by_key(|r| {
        sort_operations_counter += 1;
        r.width
    });
    println!("{:?}", rect_list[0].height);
    println!("{rect_list:?}");

    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}
