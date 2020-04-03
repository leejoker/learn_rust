//! this is a demo lib

///init_a_int_value is a method to create a i32 value
///
/// # Examples
///
/// ```
/// let  a = 5;
/// assert_eq!(a,cargo_learn::init_a_int_value(5));
///
/// ```
pub fn init_a_int_value(value: i32) -> i32 {
    value
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod kinds {
    /// 采用 RGB 色彩模式的主要颜色。
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
}
