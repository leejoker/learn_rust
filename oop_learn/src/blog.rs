/// 这部分代码使用的是状态模式，所有状态的改变对于Post本身而言都是未知的，进行的行为都取决于当前State的值
/// 当然，书中还有另一种形式，状态的转移并不是通过定义Trait去实现，而是通过定义不同的struct，通过类型的变化来进行类型的转换，
/// 不过这种方式依赖了rust所有权的特性，通过所有权的转移确保只能存在一种状态


trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct PendingReview {}

impl State for PendingReview {
    //审核改变状态为 PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //审核通过改变状态
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Draft {}

impl State for Draft {
    //进行审核
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    //审核通过
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

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
        self.content.push_str(text);
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

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
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

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        &_post.content
    }
}
