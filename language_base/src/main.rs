fn main() {
    //不可变变量
    let x = "123";
    println!("The value of x is : {}", x);
    //变量隐藏
    let x = "456";
    println!("The value of x is : {}", x);
    //可变变量，变量隐藏
    let mut x = "789";
    println!("The value of x is : {}", x);
    //可变变量重新赋值
    x = "666";
    println!("The value of x is : {}", x);

    //声明常量
    const MAX_POINTS: u64 = 100_000_000;
    println!("The value of MAX_POINTS is : {}", MAX_POINTS);

    //变量隐藏
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is : {}", x);

    //类型可变和不可变
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces length is : {}", spaces);

    // let mut spaces = "   ";
    //重新赋值会报错，类型不可改变
    // spaces = spaces.len();

    // 加法
    let sum = 5 + 10;
    println!("sum : {}", sum);

    // 减法
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // 乘法
    let product = 4 * 30;
    println!("product: {}", product);

    // 除法
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);

    // 取余
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    //元组
    let tup_with_type: (i32, f64, u8) = (500, 6.4, 1);
    println!(
        "tup_with_type : {},{},{}",
        tup_with_type.0, tup_with_type.1, tup_with_type.2
    );
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {},{},{}", x, y, z);

    //数组，固定长度
    let a = [1, 2, 3, 4, 5];
    println!("Array length is {}", a.len());
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array with Type {},{}", a[0], a[1]);

    //函数调用
    another_function();
    another_function_with_parameter(15);

    //包含语句和表达式的函数体
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value x is {}", x);
    println!("The value y is {}", y);

    //具有返回值的函数
    let x = return_number();
    println!("Function return value is : {}", x);
    let x = number_plus(100, 200);
    println!("number_plus return value is : {}", x);

    //控制流和赋值，各分支返回值类型相同
    let x = if x > 300 {
        println!("Value bigger than 300");
        500
    } else if x == 300 {
        println!("Value equals 300");
        300
    } else {
        println!("Value smaller than 300");
        200
    };
    println!("Value of x is : {}", x);

    //loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("loop result value: {}", result);
    //while
    counter = 0;
    while counter < 10 {
        counter += 1;
    }
    println!("while result value: {}", result);

    //for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("element value is : {}", element);
    }
    for element in (1..10).rev() {
        println!("element value is : {}", element);
    }
}

fn another_function() {
    println!("Another Function");
}

fn another_function_with_parameter(x: i32) {
    println!("Another Function with Parameter : {}", x);
}

fn return_number() -> i32 {
    5
}

fn number_plus(x: i64, y: i64) -> i64 {
    x + y
}
