use ch17_3_types::*;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.reject();

    let mut post = post.request_review();

    post.approve();

    let post = post.reject();

    let mut post = post.request_review();

    post.approve();
    post.approve();

    let post = post.publish().into_post().expect("Not a Post");

    assert_eq!("I ate a salad for lunch today", post.content());
}
