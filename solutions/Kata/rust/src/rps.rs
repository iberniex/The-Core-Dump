use std::collections::HashMap;

pub fn rps<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if p1 == p2 {
        return "Draw!";
    }
    let beats: HashMap<&str, &str> = HashMap::from([
        ("rock", "scissors"),
        ("scissors", "paper"),
        ("paper", "rock"),
    ]);

    if beats.get(p1) == Some(&p2) {
        return "Player 1 won!";
    }

    "Player 2 won!"
}

pub fn better_rps<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if p1 == p2 {
        return "Draw!";
    }

    match (p1, p2) {
        ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => "Player 1 won!",
        _ => "Player 2 won!",
    }
}

#[cfg(test)]
mod tests {
    use crate::rps::better_rps;

    use super::rps;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p1: &str, p2: &str, expected: &str) {
        assert_eq!(
            rps(p1, p2),
            expected,
            "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\""
        );
        assert_eq!(
            better_rps(p1, p2),
            expected,
            "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("rock", "scissors", "Player 1 won!");
        dotest("scissors", "rock", "Player 2 won!");
        dotest("rock", "rock", "Draw!");
    }
}
