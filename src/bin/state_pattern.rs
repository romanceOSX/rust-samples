// --> file:///Users/romance/.rustup/toolchains/stable-aarch64-apple-darwin/share/doc/rust/html/book/ch18-03-oo-design-patterns.html

/*
 * - A blog starts with an empty draft
 * - when the draft is done, a review of the post is requested
 * - when the post is approved, it gets published
 * - Only published blog posts return the content to print 
 */

// TODO: compare this implementation to the enum's
mod blog;
mod rust_blog;

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


use rust_blog::PostRust;

fn test_rust_approach() {
    let mut post = PostRust::new();
    post.add_text("This is a post");
    let post = post.request_review();
    let post = post.approve();
    println!("Printing the post's content \"{}\"", post.content());
}

fn main() {
    test_first_approach();
    test_rust_approach();
    println!("testing state pattern");
}

