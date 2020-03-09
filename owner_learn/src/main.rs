fn main() {
    //所有权
    let a = 4;
    let b = a;
    //这里也存在拷贝，不过发生在栈上
    println!("{},{}", a, b);

    let c = "aaa";
    let d = c;

    //类型为&str，是不可变引用，会被直接保存在文件中
    println!("{}", c);
    println!("{}", d);

    //本质是vector，执行时才在堆上分配
    let e = String::from("bbb");
    let f = e;

    //println!("{}",e);      会报错，所有权转移，e被抛弃
    println!("{}", f);

    //深度拷贝
    let e = String::from("bbb");
    let f = e.clone();
    println!("{}", f);

    //在函数中使用后也会发生所有权转移
    make_owner_change(f);
    //println!("{}",f);      这里也会报错
    let g = make_owner_change(e);
    println!("{}", g);
    //println!("{}",e);      e的所有权已经转移

    //可以通过传递引用的方式避免所有权转移
    let h = String::from("ccc");
    let j = use_reference(&h);
    println!("h: {}, j: {}", h, j);

    //可变引用
    let mut k = String::from("hello");
    change(&mut k);
    println!("value has changed: {}", k);

    //可变引用借用
    let k1 = &mut k;
    //let k2 = &mut k;       可变引用在特定作用域中只能借用一次（前提：所有权未变更）
    println!("{}", k1);

    /*
    如果进行了不可变借用，不允许再进行可变借用

    let mut l = String::from("aloha");
    let l1 = &l;
    let l2 = &l;
    let l3 = &mut l;
    println!("{},{},{}", l1, l2, l3);
     */

    //slice
    let slice_str = "Hello World!";
    let slice1 = &slice_str[0..5];
    let slice2 = &slice_str[6..];
    println!("slice1 is {}, slice2 is {}", slice1, slice2);
}

fn make_owner_change(str: String) -> String {
    str
}

fn use_reference(str: &String) -> String {
    String::from(str)
}

fn change(str: &mut String) {
    str.push_str(", world");
}

