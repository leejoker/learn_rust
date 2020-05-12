use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Car {
    pub name: String,
    pub whells: RefCell<Vec<Weak<Wheel>>>,
}

#[derive(Debug)]
pub struct Wheel {
    pub id: i32,
    pub car: Rc<Car>,
}
