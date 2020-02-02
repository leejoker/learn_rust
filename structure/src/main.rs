//结构体
#[derive(Debug)]
struct User {
    username: String,
    sex: u8,
    age: u8,
    email: String,
    active: bool,
}

//定义结构体方法
impl User {
    fn show_sex(&self) -> String {
        if self.sex == 1 {
            String::from("男")
        } else {
            String::from("女")
        }
    }

    fn is_older(&self, other: &User) -> bool {
        self.age > other.age
    }

    fn set_email(email: String) -> User {
        User {
            username: String::from("default"),
            sex: 1,
            age: 20,
            email,
            active: true,
        }
    }
}

//元组结构体
struct Color(i32, i32, i32);

fn main() {
    //创建不可变变量user1
    let user1 = User {
        username: String::from("leejoker"),
        sex: 1,
        age: 29,
        email: String::from("liang91123@gmail.com"),
        active: true,
    };

    println!(
        "create user1: {}, {}, {}, {}, {}",
        user1.username, user1.sex, user1.age, user1.email, user1.active
    );

    //创建可变变量user2
    let mut user2 = build_user(String::from("lee"), 1, 23, String::from("lee@163.com"));
    user2.email = String::from("lee_change@163.com");

    println!(
        "create mut user2: {}, {}, {}, {}, {}",
        user2.username, user2.sex, user2.age, user2.email, user2.active
    );

    //通过user2的值创建user3
    let user3 = User {
        username: user2.username,
        sex: 0,
        age: user2.age,
        email: user2.email,
        active: user2.active,
    };

    println!(
        "create mut user3: {}, {}, {}, {}, {}",
        user3.username, user3.sex, user3.age, user3.email, user3.active
    );

    //另一种方式复用user3的值
    let user4 = User {
        username: String::from("new_user"),
        ..user3
    };

    println!(
        "create mut user4: {}, {}, {}, {}, {}",
        user4.username, user4.sex, user4.age, user4.email, user4.active
    );

    let black = Color(0, 0, 0);
    println!("color is {}, {}, {}", black.0, black.1, black.2);

    println!("try to show user4: {:?}", user4);
    println!("show user4 in another way: {:#?}", user4);
    println!("show user4 sex: {}", user4.show_sex());
    println!("user4 is older than user1: {}", user4.is_older(&user1));

    let email_user = User::set_email(String::from("test@email.com"));
    println!("associated function test: {:#?}", email_user);
}

fn build_user(username: String, sex: u8, age: u8, email: String) -> User {
    User {
        username,
        sex,
        age,
        email,
        active: true,
    }
}
