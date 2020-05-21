use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn send_and_receive() {
    let (tx, rx) = mpsc::channel();

    //克隆发送者
    let tx_clone = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let val = String::from("hi");
        //调用send方法会转移val的所有权
        tx.send(val).unwrap();

        let vals = vec![
            String::from("leejoker"),
            String::from("in"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //recv方法阻塞主线程直到获取值   try_recv不会阻塞主线程
    // let received = rx.recv().unwrap();
    for received in rx {
        println!("Got: {}", received);
    }
}
