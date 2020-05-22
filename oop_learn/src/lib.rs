/// Trait对象
/// ## trait 对象是安全的判定原则：
/// 1. 返回值类型不为 Self
/// 2. 方法没有任何泛型类型参数
///
/// ## 使用泛型限制类型，只能使用单一类型
/// ```
/// pub trait Draw {
///     fn draw(&self);
/// }
///
/// pub struct Screen<T: Draw> {
///     pub components: Vec<Box<T>>,
/// }
///
/// impl<T> Screen<T>
/// where
///    T: Draw,
/// {
///     pub fn run(&self) {
///        for component in self.components.iter() {
///           component.draw();
///        }
///     }
/// }
/// ```
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button {}", self.label);
    }
}

pub struct Section {
    pub width: u32,
    pub height: u32,
    pub color: String,
}

impl Draw for Section {
    fn draw(&self) {
        println!("Draw Section with color {}", self.color);
    }
}

#[cfg(test)]
mod tests {
    use super::Button;
    use super::Screen;
    use super::Section;

    //cargo test -- --nocapture 显示输出信息
    #[test]
    fn run_tests() {
        let screen = Screen {
            components: vec![
                Box::new(Button {
                    width: 10,
                    height: 5,
                    label: String::from("button1"),
                }),
                Box::new(Section {
                    width: 20,
                    height: 15,
                    color: String::from("Red"),
                }),
            ],
        };

        screen.run();
    }
}
