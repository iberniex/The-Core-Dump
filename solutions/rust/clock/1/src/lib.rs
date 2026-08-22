
#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let total_minutes = hours * 60 + minutes;
        Self::from_minutes(total_minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        let total_minutes = self.hours * 60 + self.minutes + minutes;

        Self::from_minutes(total_minutes)
        
    }

    pub fn from_minutes(minutes: i32) -> Self {
        let positive_minutes = minutes.rem_euclid(24 * 60);

        let hours = (positive_minutes / 60) % 24;
        let minutes = positive_minutes % 60;

        Self { hours, minutes }
    }
}
