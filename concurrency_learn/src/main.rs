mod message_passing;
mod mutex_demo;
mod thread_demo;

fn main() {
    thread_demo::thread_demo();
    message_passing::send_and_receive();
    mutex_demo::mutex_lean();
    mutex_demo::mutex_share();
}
