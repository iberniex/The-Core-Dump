pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    // candies
    //     .clone()
    //     .into_iter()
    //     .map(|x| {
    //         let proposed_sol = x + extra_candies;
    //         match candies.iter().max() {
    //             Some(&val) => {
    //                 if proposed_sol >= val {
    //                     true
    //                 } else {
    //                     false
    //                 }
    //             }
    //             _ => false,
    //         }
    //     })
    //     .collect()
    let max = candies.iter().max().unwrap_or(&0);
    candies
        .iter()
        .map(|candy| candy - max + extra_candies >= 0)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::greatest_number_of_candies::kids_with_candies;

    #[test]
    fn candies_uwu() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }
    #[test]
    fn more_candies() {
        assert_eq!(
            kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }
}
