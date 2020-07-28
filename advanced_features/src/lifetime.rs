pub struct Context<'s>(&'s str);

//声明s的生命周期不短于c
pub struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    pub fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

pub fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

pub struct Ref<'a, T: 'a>(&'a T);

//通过匿名生命周期消除噪音
pub struct StrWrap<'a>(&'a str);
fn foo(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}
