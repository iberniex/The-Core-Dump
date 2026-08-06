pub fn factorial(value: u64) -> Option<u64> {
    (1..=value).try_fold(1u64, |x, acc| x.checked_mul(acc))
}
