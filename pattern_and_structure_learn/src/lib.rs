///==========================match分支=================================
/// ```
///     matche VALUE {
///         PATTERN => EXPRESSION,
///         PATTERN => EXPRESSION,
///         PATTERN => EXPRESSION,
///     }
/// ```
/// match表达式应该尽量覆盖所有的可能，做到穷尽。

#[cfg(test)]
mod tests {
    #[test]
    fn if_let_test() {
        //===========================if let=============================
        let favorite_color: Option<&str> = None; // or  Option::from("gray");   you can see the differents
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    #[test]
    fn while_let_test() {
        //========================= while let =============================
        let mut stack: Vec<i32> = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    #[test]
    fn for_test() {
        //========================= for =============================
        let v = vec![1, 2, 3, 4, 5];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    #[test]
    fn let_test() {
        //========================= let ===============================
        let x = 5;
        println!("{}", x);
        let (x, y, z) = (1, 2, 3);
        println!("{} {} {}", x, y, z);
    }

    #[test]
    fn function_params_test() {
        let position = (3, 5);
        _coordinates(&position);
    }

    fn _coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({},{})", x, y);
    }
}
