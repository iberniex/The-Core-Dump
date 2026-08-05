pub mod hashmaps {
    use std::collections::HashMap;

    pub fn word_count() {
        let long_string = String::from("hello world wonderful world");

        let mut mappy = HashMap::new();

        for word in long_string.split_whitespace() {
            let inserty = mappy.entry(word).or_insert(0);
            *inserty += 1;
        }

        for (key, value) in mappy {
            print!("{} - {}\n", key, value);
        }
    }
}
