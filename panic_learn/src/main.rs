use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];
    let f = File::open("/home/leejoker/downloads/test");

    // match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file {:?}", error);
    //     }
    // };

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("/home/leejoker/downloads/test") {
                Ok(fc) => fc,
                Err(e) => panic!("Tired to create file but there was a problem {:?}", e),
            },
            other_error => panic!("There was a problem opening the file {:?}", other_error),
        },
    };

    println!("file is {:?}", f);

    let f = File::open("/home/leejoker/downloads/test2").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("/home/leejoker/downloads/test2").unwrap_or_else(|error| {
                panic!("Tired to create file but there was a problem {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file {:?}", error);
        }
    });

    println!("file is {:?}", f);

    //直接捕获异常并输出
    // File::open("/home/leejoker/downloads/test3").unwrap();
    //使用expect获取自定义异常信息
    // File::open("/home/leejoker/downloads/test3").expect("Failed to open test3");

    let re = read_username_from_file();
    match re {
        Ok(s) => println!("read value is {}", s),
        Err(e) => {
            panic!("There was a problem reading the username {:?}", e);
        }
    }
}

//传播错误简写的方式只能被用于返回Result的函数中
fn read_username_from_file() -> Result<String, Error> {
    // let f = File::open("/home/leejoker/downloads/test3");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    //another way
    // let mut f = File::open("/home/leejoker/downloads/test3")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    //chain
    let mut s = String::new();
    File::open("/home/leejoker/downloads/test3")?.read_to_string(&mut s)?;
    Ok(s)
}
