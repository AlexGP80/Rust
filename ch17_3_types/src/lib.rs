pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
        }
    }
}

pub enum ApprovalResult {
    PendingReviewPost(PendingReviewPost),
    Post(Post),
}

impl ApprovalResult {
    pub fn into_pending_post(self) -> Option<PendingReviewPost> {
        use ApprovalResult::*;
        match self {
            PendingReviewPost(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_post(self) -> Option<Post> {
        use ApprovalResult::*;
        match self {
            Post(v) => Some(v),
            _ => None,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approvals: i32,
}

impl PendingReviewPost {
    pub fn publish(self) -> ApprovalResult {
        if self.approvals >= 2 {
            ApprovalResult::Post(Post {
                content: self.content,
            })
        } else {
            ApprovalResult::PendingReviewPost(self)
        }
    }

    pub fn approve(&mut self) {
        self.approvals += 1;
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
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
