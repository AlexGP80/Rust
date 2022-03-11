/// Post
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
        self.content = self.state.as_ref().unwrap().add_text(&self.content, text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
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
            self.state = Some(s.reject())
        }
    }
}

/// State (trait)
trait State {
    /// Note that rather than having self, &self, or &mut self as the first parameter of the
    /// method, we have self: Box<Self>. This syntax means __the method is only valid when called__
    /// __on a Box holding the type__.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    /// Default implementation for the content method that returns an empty string slice
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }

    /// Default implementation for the add_text method that returns the original text as String
    fn add_text(&self, original_text: &str, _: &str) -> String {
        original_text.to_string()
    }
}

/// Draft (implements State)
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, original_text: &str, text_to_add: &str) -> String {
        format!("{}{}", original_text, text_to_add)
    }
}

/// PendingReview (implements State)
struct PendingReview {
    number_of_approvals: i32,
}

impl PendingReview {
    fn new() -> PendingReview {
        PendingReview {
            number_of_approvals: 0,
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.number_of_approvals += 1;
        if self.number_of_approvals == 2 {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

/// Published (implements State)
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // why can we access to content field?
        &post.content
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
