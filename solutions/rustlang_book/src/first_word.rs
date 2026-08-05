pub mod first_word {

    pub fn main() {
        let s1 = String::from("hello world");
        let slice = &s1[5..];
        let first_word_length = first_word(&s1);
        println!("This is the first word length of {s1} : {first_word_length}");

        println!("Slicing '{s1}': {slice}");
    }

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}
