use time::{PrimitiveDateTime as DateTime, SignedDuration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let Some(date) = start.checked_add(SignedDuration::new(1000000000, 0)) else {
        return start;
    };

    date
}
