use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        // println!("Today, do {} pushups!",expensive_closure(intensity));
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        // println!("Next, do {} situps!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            // println!("Today, run for {} minutes!", expensive_closure(intensity));
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }

    //闭包的类型需要保持一直
    //expensive_closure(String::from("asdasdad"));
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    //一旦被赋值后无法改变
    //let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn equal_with_values() {
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("{}", x);
    let y = 4;
    assert!(equal_to_x(y));
}

#[test]
fn equal_with_move() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    //move转移了所有权，x无法再被使用
    //println!("try to use x here {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

pub struct Count {
    value: i32,
}

impl Count {
    pub fn new() -> Count {
        Count { value: 0 }
    }
}

impl Iterator for Count {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        if self.value < 6 {
            Some(self.value)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Count::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
