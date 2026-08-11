pub fn gcd_of_strings(string_pat: String, string_child: String) -> String {
    let gcd = |mut a: usize, mut b: usize| {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    };

    let lhs_string = string_pat.clone() + &string_child;
    let rhs_string = string_child.clone() + &string_pat;

    if lhs_string != rhs_string {
        return String::new();
    }

    string_pat[..gcd(string_pat.len(), string_child.len())].to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_gcd_strings() {
        assert_eq!(
            gcd_of_strings(String::from("ABCABCABCD"), String::from("ABC")),
            String::from("")
        );

        assert_eq!(
            gcd_of_strings(String::from("ABCABCABC"), String::from("ABC")),
            String::from("ABC")
        );
    }
}
