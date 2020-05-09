use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
pub enum RcList {
    Cons2(i32, Rc<RcList>),
    None,
}
