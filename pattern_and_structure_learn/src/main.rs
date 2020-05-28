fn main() {
    //================== 匹配字面量 ========================
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //================= 匹配命名变量 =======================
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    //================= 匹配多个模式 =========================
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //================= 匹配范围值 =========================
    let x = 4;
    let y = '😀';

    match x {
        1..=5 => println!("one through five, value is {}", x),
        _ => println!("something else"),
    }

    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //====================== 解构 ===========================
    let p = Point { x: 3, y: 5 };
    let Point { x, y } = p;
    println!("point x={},y={}", x, y);
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    //枚举解构请参考enum_learn的代码
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("sum is {}", sum_of_squares);

    //使用 _ 进行忽略，或者表示不处理
    let _aaa = 10; //在变量名前增加_，不使用也不会报错

    //使用 .. 进行多值的忽略，但使用不能存在歧义
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    //match guard
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    //==================== bind ===========================
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}
