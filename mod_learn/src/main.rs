//use std::fmt::Result;
// use std::io::Result as IResult;
// use std::{fmt::Result, io};
// use std::io::{self, Write};
// use std::collections::*;

//声明了mod后，建立同名文件即可，不需要在文件内部再次声明这个mod
mod plant;

mod sound {
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                println!("I will do something in this function");
            }
        }
    }

    pub fn call_in_father() {
        super::do_in_father();
    }
}

fn do_in_father() {
    println!("Do something in father");
}

fn main() {
    //绝对路径
    crate::sound::instrument::woodwind::clarinet();
    //相对路径
    sound::instrument::woodwind::clarinet();

    crate::sound::call_in_father();

    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternet squash");
    println!("{} are delicious", v.name);
}
