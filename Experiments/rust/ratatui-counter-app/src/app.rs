#[derive(Debug, Default)]
pub struct App {
    /// Application exit session handler.
    pub should_quit: bool,

    /// Counter
    pub counter: u8,
}

impl App {
    /// Construct a new instance of [`App`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of this terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.increment_counter();
        app.increment_counter();
        app.decrement_counter();
        assert_eq!(app.counter, 1);
    }
}
