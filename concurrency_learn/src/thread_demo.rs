use std::thread;
use std::time::Duration;

pub fn thread_demo() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread !", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread !", i);
        thread::sleep(Duration::from_millis(2));
    }

    handle.join().unwrap();

    for i in 1..5 {
        println!("hey number {} from the main thread !", i);
        thread::sleep(Duration::from_millis(2));
    }

    let v = vec![1, 2, 3];

    //使用move进行主线程和子线程的参数传递，转移所有权
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
