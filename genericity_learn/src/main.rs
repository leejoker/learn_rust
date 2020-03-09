use genericity::Summary;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<i32, f32> {
    fn mix_add(&mut self) {
        self.y = self.x as f32 + self.y;
    }
}

impl Summary for Point<i32, f32> {
    fn summarize(&self) -> String {
        format!("{}", self.x as f32 + self.y)
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let mut both = Point {
        x: 5,
        y: 6.7,
    };

    println!("x value is {}, y value is {}", both.x(), both.y());

    both.mix_add();

    println!("{:#?}\n{}\n{}", both, both.x, both.y);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let str = both.summarize();
    println!("{}", str);


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
