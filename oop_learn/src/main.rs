mod blog;

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch tody");
    assert_eq!("", post.content());
}
