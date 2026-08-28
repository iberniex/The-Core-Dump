pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state.as_ref().unwrap().add_text(&self.content, text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // take() take the value inside the box so that it can be consumed
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.draft())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn draft(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn add_text(&self, current_text: &str, _text: &str) -> String {
        current_text.to_string()
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn draft(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text(&self, current_text: &str, text: &str) -> String {
        format!("{}{}", text, current_text)
    }
}

struct PendingReview {
    approvals: u8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approvals += 1;

        if self.approvals >= 2 {
            return Box::new(Published {});
        } else {
            self
        }
    }

    fn draft(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn draft(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_post() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
