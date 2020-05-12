mod lib;
mod mybox;
mod rcweark;
use lib::List::{Cons, Nil};
use lib::RcList::{Cons2, None};
use rcweark::Car;
use rcweark::Wheel;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{:#?}", list);

    println!("---------------------------------------------------------------------解引用-------------------------------------------------------------------");

    let x = 5;
    let y = &x;

    //assert_eq!(x, y);   error:  can't compare `{integer}` with `&{integer}`
    assert_eq!(x, *y); //解引用

    let y = Box::new(x);
    assert_eq!(x, *y);

    println!("---------------------------------------------------------------------Box和析构-------------------------------------------------------------------");

    let y = mybox::MyBox::new(x);
    assert_eq!(x, *y);
    drop(y); //手动析构

    {
        let x = mybox::MyBox::new(15);
        println!("inner value: {}", *x);
        //离开作用域后自动析构
    }

    let name = mybox::MyBox::new(String::from("Leejoker"));
    mybox::hello(&name);

    println!("---------------------------------------------------------------------Rc引用计数-------------------------------------------------------------------");

    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(None)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // println!("RcList Value: {:#?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // println!("RcList Value: {:#?}", b);

    println!("---------------------------------------------------------------------内部可变模式-------------------------------------------------------------------");

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    println!("---------------------------------------------------------------------Rc And Weak-------------------------------------------------------------------");

    //Rc 和 Weak
    let car: Rc<Car> = Rc::new(Car {
        name: "A".to_string(),
        whells: RefCell::new(vec![]),
    });
    let whell = Rc::new(Wheel {
        id: 1,
        car: Rc::clone(&car),
    });

    print_strong_and_weak(&car, &whell);

    let mut whells = car.whells.borrow_mut();
    whells.push(Rc::downgrade(&whell));
    drop(whells);

    print_strong_and_weak(&car, &whell);

    for whell_weak in car.whells.borrow().iter() {
        let whell = whell_weak.upgrade().unwrap();
        println!("-----Whell {} owned by {}-------", whell.id, whell.car.name);
        print_strong_and_weak(&car, &whell);
    }

    print_strong_and_weak(&car, &whell);

    // println!("{:#?}", car);
    // println!("{:#?}", whell);
}

fn print_strong_and_weak(car: &Rc<Car>, wheel: &Rc<Wheel>) {
    println!(
        "car strong count: {},car weak count: {}",
        Rc::strong_count(car),
        Rc::weak_count(car)
    );
    println!(
        "whell strong count: {},whell weak count: {}",
        Rc::strong_count(wheel),
        Rc::weak_count(wheel)
    );
}
