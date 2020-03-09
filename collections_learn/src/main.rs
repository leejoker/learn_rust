use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCall {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("third value is {}", third);

    match v.get(3) {
        Some(fourth) => println!("fourth value is {}", fourth),
        None => println!("There is no fourth value"),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCall::Int(3),
        SpreadsheetCall::Float(3.6),
        SpreadsheetCall::Text(String::from("blue")),
    ];

    for i in &row {
        println!("{:#?}", i);
    }

    // ---------------------------------- String ------------------------------------
    let mut s = String::new();
    s.push_str("hahahah");
    println!("s is {}", s);
    let data = "initial contents";
    let s = data.to_string();
    println!("s is {}", s);
    let s = "initial contents".to_string();
    println!("the value s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("result value is {}", s);

    for c in s.chars() {
        println!("{}", c);
    }

    //------------------------- hashmap -----------------------------------
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);
    println!("{:?}", &scores);
    //更新value
    scores.insert(String::from("blue"), 60);
    println!("{:?}", scores);

    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    //处理获取的值
    match score {
        Some(value) => println!("score is {}", value),
        None => println!("there is nothing to print"),
    }

    //遍历hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.entry(String::from("yellow")).or_insert(80);
    println!("{:?}", scores);

    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
