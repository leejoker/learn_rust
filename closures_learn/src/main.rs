use closures_learn;

fn main() {
    //闭包
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    closures_learn::generate_workout(simulated_user_specified_value, simulated_random_number);

    //迭代器
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}
