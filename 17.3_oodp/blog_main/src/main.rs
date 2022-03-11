use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // This will be ignored because State is PendingReview instead of Draft
    post.add_text(". This night I'll go to MacDonnald's.");
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    // Note there should be no text about MacDonnald's
    assert_eq!("I ate a salad for lunch today", post.content());
}
