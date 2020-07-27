use crate::consts::{COUNTER, HELLO_WORLD};
use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn unsafe_tests() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    println!("a is {:?},b is {:?}", a, b);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("static var is {}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

//创建一个可以被c调用的函数
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

//修改可变静态变量
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
