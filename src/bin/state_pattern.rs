// --> file:///Users/romance/.rustup/toolchains/stable-aarch64-apple-darwin/share/doc/rust/html/book/ch18-03-oo-design-patterns.html

/*
 * - A blog starts with an empty draft
 * - when the draft is done, a review of the post is requested
 * - when the post is approved, it gets published
 * - Only published blog posts return the content to print 
 */

mod blog;

use blog::Post;

fn test_first_approach() {
    let mut post = Post::new();

    post.add_text("I ate a Super Salad's Santa Fe salad");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a Super Salad's Santa Fe salad", post.content());
}

fn main() {
    test_first_approach();
    println!("testing state pattern");
}

