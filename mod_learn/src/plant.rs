pub struct Vegetable {
    pub name: String,
    //如果声明了私有属性，外部不可访问
    // id: i32,
}

impl Vegetable {
    pub fn new(name: &str) -> Vegetable {
        Vegetable {
            name: String::from(name),
        }
    }
}
